import SwiftRs
import Tauri
import UIKit
import WebKit

class ExamplePlugin: Plugin {
    
    // MARK: - Rust FFI Definitions
    @_silgen_name("push_intent_ffi")
    private static func pushIntentFFI(_ name: UnsafePointer<CChar>)
    
    // MARK: - Lifecycle Handling
        
        // This is called when the plugin is loaded
        override func load(webview: WKWebView) {
            // 1. Check immediately on startup
            self.processPendingIntents()

            // 2. Listen for when the app comes to the foreground (user switches back from another app)
            NotificationCenter.default.addObserver(
                self,
                selector: #selector(appDidBecomeActive),
                name: UIApplication.didBecomeActiveNotification,
                object: nil
            )
        }

        @objc func appDidBecomeActive() {
            self.processPendingIntents()
        }
    
    // MARK: - Logic
        
        private func processPendingIntents() {
            // 1. Connect to the same App Group
            guard let sharedDefaults = UserDefaults(suiteName: "group.com.tauri.share.extension") else { return }
            
            // 2. Fetch the queue
            guard let intentQueue = sharedDefaults.stringArray(forKey: "shared_intent_queue"), !intentQueue.isEmpty else {
                return
            }
            
            // 3. Loop through and push to Rust
            for urlString in intentQueue {
                print("Pushing intent to Rust: \(urlString)")
                urlString.withCString { ptr in
                    ExamplePlugin.pushIntentFFI(ptr)
                }
            }
            
            // 4. Clear the queue in storage so we don't process them again
            sharedDefaults.set([String](), forKey: "shared_intent_queue")
            sharedDefaults.synchronize()
        }
}



@_cdecl("init_plugin_mobile_sharetarget")
func initPlugin() -> Plugin {
      return ExamplePlugin()
}
