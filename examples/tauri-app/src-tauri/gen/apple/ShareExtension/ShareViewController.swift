//
//  ShareViewController.swift
//  ShareExtension
//
//  Created by Itess on 28/11/2025.
//

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
                // e.g., myapp://share?link=https%3A%2F%2Fgoogle.com
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
            let alert = UIAlertController(title: "Error", message: "Could not open the main app.", preferredStyle: .alert)
            alert.addAction(UIAlertAction(title: "OK", style: .default, handler: { _ in
                self.closeExtension()
            }))
            self.present(alert, animated: true)
        }

        // MARK: - Data Extraction
        private func extractSharedURL(completion: @escaping (URL?) -> Void) {
            // Safely unwrap extension items
            guard let extensionItem = extensionContext?.inputItems.first as? NSExtensionItem,
                  let attachments = extensionItem.attachments else {
                completion(nil)
                return
            }
            
            // Look for a URL provider
            // Note: We use UTType.url.identifier (modern) or kUTTypeURL (legacy)
            let typeIdentifier = UTType.url.identifier // "public.url"
            
            for provider in attachments {
                if provider.hasItemConformingToTypeIdentifier(typeIdentifier) {
                    
                    // Load item safely
                    provider.loadItem(forTypeIdentifier: typeIdentifier, options: nil) { (item, error) in
                        // This runs on a background thread
                        DispatchQueue.main.async {
                            if let error = error {
                                print("🔴 Load Error: \(error.localizedDescription)")
                            }
                            
                            // Handle the weird ways iOS returns URLs (sometimes NSURL, sometimes URL)
                            if let url = item as? URL {
                                completion(url)
                            } else if let url = item as? NSURL {
                                completion(url as URL)
                            } else {
                                completion(nil)
                            }
                        }
                    }
                    return // Found one, stop looking
                }
            }
            completion(nil) // No URL found
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
