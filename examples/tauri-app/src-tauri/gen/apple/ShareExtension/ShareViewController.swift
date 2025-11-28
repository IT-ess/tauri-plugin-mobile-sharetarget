//
//  ShareViewController.swift
//  ShareExtension
//
//  Created by Itess on 28/11/2025.
//

import UIKit
import Social
import MobileCoreServices

class ShareViewController: SLComposeServiceViewController {

    override func isContentValid() -> Bool {
        // Validation logic (e.g., check if it's a valid URL)
        return true
    }

    override func didSelectPost() {
        // This is called after the user presses "Post/Share"
        
        guard let extensionContext = extensionContext else { return }
        
        let attachments = extensionContext.inputItems.first as? NSExtensionItem
        let contentType = kUTTypeURL as String // We are looking for URLs
        
        if let provider = attachments?.attachments?.first {
            if provider.hasItemConformingToTypeIdentifier(contentType) {
                provider.loadItem(forTypeIdentifier: contentType, options: nil) { [weak self] (data, error) in
                    guard error == nil else { return }
                    
                    if let url = data as? URL {
                        self?.saveToAppGroup(url: url.absoluteString)
                    }
                    
                    // Inform the host that we're done, so it unblocks the UI
                    self?.extensionContext?.completeRequest(returningItems: [], completionHandler: nil)
                }
            }
        }
    }

    private func saveToAppGroup(url: String) {
        // 1. Access the Shared User Defaults
        guard let sharedDefaults = UserDefaults(suiteName: "group.com.tauri.share.extension") else {
            print("Error: Could not load App Group")
            return
        }
        
        // 2. Read existing queue or create new one
        var intentQueue = sharedDefaults.stringArray(forKey: "shared_intent_queue") ?? []
        
        // 3. Append new URL
        intentQueue.append(url)
        
        // 4. Save back
        sharedDefaults.set(intentQueue, forKey: "shared_intent_queue")
        sharedDefaults.synchronize()
    }
}
