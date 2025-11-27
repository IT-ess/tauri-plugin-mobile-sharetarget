import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {
  @objc public func ping(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }
    
    @objc public func testHelloWorld(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(PingArgs.self)
        let name = args.value ?? "World"
        
        if let result = ExamplePlugin.helloWorld(name: name) {
          invoke.resolve(["value": result])
        } else {
          invoke.reject("Failed to call helloWorld")
        }
      }
    
    @_silgen_name("hello_world_ffi")
    private static func helloWorldFFI(_ name: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

    @_silgen_name("free_hello_result_ffi")
    private static func freeHelloResult(_ result: UnsafeMutablePointer<CChar>)

    static func helloWorld(name: String) -> String? {
      // Call Rust FFI
      let resultPtr = name.withCString({ helloWorldFFI($0) })

      // Convert C string to Swift String
        let result = String(cString: resultPtr.unsafelyUnwrapped)

      // Free the C string
        freeHelloResult(resultPtr.unsafelyUnwrapped)

      return result
    }
}



@_cdecl("init_plugin_mobile_sharetarget")
func initPlugin() -> Plugin {
      return ExamplePlugin()
}
