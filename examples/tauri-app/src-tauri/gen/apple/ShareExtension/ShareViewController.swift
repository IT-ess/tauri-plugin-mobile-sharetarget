//
//  ShareViewController.swift
//  ShareExtension
//
//  Created by Itess on 28/11/2025.
//

import Foundation
import UIKit
import UniformTypeIdentifiers

class ShareViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Minimal UI: Transparent with spinner
        self.view.backgroundColor = .clear
        let spinner = UIActivityIndicatorView(style: .large)
        spinner.center = self.view.center
        spinner.startAnimating()
        self.view.addSubview(spinner)
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        print("🟢 Share Extension: View Did Appear")

        // 1. Extract Data safely
        extractSharedURL { [weak self] sharedURL in
            guard let self = self else { return }

            guard let url = sharedURL else {
                print("🔴 Share Extension: No URL found in shared content.")
                self.closeExtension()
                return
            }

            // 2. Build Deeplink (WITH ENCODING)
            // If the shared URL has special chars, it MUST be encoded or URL(string:) returns nil
            let originalString = url.absoluteString

            // Prepare the query item
            // e.g., myapp://share?url=https%3A%2F%2Fgoogle.com
            var components = URLComponents()
            components.scheme = "tauri-share"
            components.host = "share"
            components.queryItems = [
                URLQueryItem(name: "url", value: originalString)
            ]

            guard let deepLink = components.url else {
                print("🔴 Share Extension: Could not construct deep link.")
                self.closeExtension()
                return
            }

            print("🟢 Share Extension: Attempting to open -> \(deepLink)")

            // 3. Attempt to Open
            let success = self.openURL(deepLink)

            if success {
                print("🟢 Share Extension: Open command sent successfully.")
                // Give the system time to switch apps before killing this extension
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                    self.closeExtension()
                }
            } else {
                print("🔴 Share Extension: Trampoline failed. Responder not found.")
                // Fallback: Show an alert so the user isn't left confusingly
                self.showErrorAndClose()
            }
        }
    }

    // MARK: - Helper Methods

    private func closeExtension() {
        self.extensionContext?.completeRequest(returningItems: [], completionHandler: nil)
    }

    private func showErrorAndClose() {
        let alert = UIAlertController(
            title: "Error", message: "Could not open the main app.", preferredStyle: .alert)
        alert.addAction(
            UIAlertAction(
                title: "OK", style: .default,
                handler: { _ in
                    self.closeExtension()
                }))
        self.present(alert, animated: true)
    }

    // MARK: - Data Extraction
    private func extractSharedURL(completion: @escaping (URL?) -> Void) {
        // Safely unwrap extension items
        guard let extensionItem = extensionContext?.inputItems.first as? NSExtensionItem,
            let attachments = extensionItem.attachments
        else {
            completion(nil)
            return
        }

        let urlType = UTType.url.identifier  // "public.url"
        let textTypes: [String] = {
            if #available(iOS 16.0, *) {
                return [UTType.text.identifier, UTType.plainText.identifier]
            } else {
                return [UTType.text.identifier]
            }
        }()

        // Helper to extract the first URL from a string using a data detector
        func urlFromString(_ string: String) -> URL? {
            let detector = try? NSDataDetector(
                types: NSTextCheckingResult.CheckingType.link.rawValue)
            let range = NSRange(location: 0, length: (string as NSString).length)
            let match = detector?.firstMatch(in: string, options: [], range: range)
            if let match = match, match.resultType == .link, let foundURL = match.url {
                return foundURL
            }
            // Fallback: direct initializer if the text is exactly a URL
            return URL(string: string.trimmingCharacters(in: .whitespacesAndNewlines))
        }

        // 1) Prefer a real URL item provider
        for provider in attachments {
            if provider.hasItemConformingToTypeIdentifier(urlType) {
                provider.loadItem(forTypeIdentifier: urlType, options: nil) { (item, error) in
                    DispatchQueue.main.async {
                        if let error = error {
                            print("🔴 Load Error (URL): \(error.localizedDescription)")
                        }
                        if let url = item as? URL {
                            completion(url)
                        } else if let url = item as? NSURL {
                            completion(url as URL)
                        } else if let string = item as? String, let url = urlFromString(string) {
                            completion(url)
                        } else {
                            completion(nil)
                        }
                    }
                }
                return
            }
        }

        // 2) Fall back to text providers that may contain a URL
        for provider in attachments {
            for type in textTypes {
                if provider.hasItemConformingToTypeIdentifier(type) {
                    provider.loadItem(forTypeIdentifier: type, options: nil) { (item, error) in
                        DispatchQueue.main.async {
                            if let error = error {
                                print("🔴 Load Error (Text): \(error.localizedDescription)")
                            }
                            if let string = item as? String, let url = urlFromString(string) {
                                completion(url)
                            } else if let data = item as? Data,
                                let string = String(data: data, encoding: .utf8),
                                let url = urlFromString(string)
                            {
                                completion(url)
                            } else {
                                completion(nil)
                            }
                        }
                    }
                    return
                }
            }
        }

        // No suitable provider found
        completion(nil)
    }

    // MARK: - The Trampoline (The Magic)
    @discardableResult
    @objc func openURL(_ url: URL) -> Bool {
        var responder: UIResponder? = self

        while responder != nil {
            if let application = responder as? UIApplication {
                application.open(url)
                return true
            }
            responder = responder?.next
        }
        return false
    }
}
