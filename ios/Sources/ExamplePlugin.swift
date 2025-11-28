import SwiftRs
import Tauri
import UIKit
import WebKit

class ExamplePlugin: Plugin {
    
}



@_cdecl("init_plugin_mobile_sharetarget")
func initPlugin() -> Plugin {
      return ExamplePlugin()
}
