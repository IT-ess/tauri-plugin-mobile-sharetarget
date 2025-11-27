package com.plugin.mobilesharetarget

import android.app.Activity
import android.util.Log
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin


@TauriPlugin
class SharetargetPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Sharetarget()

    override fun load(webView: WebView) {
        val result = implementation.helloWorld("World")
        Log.d("mobilesharetarget", "JNI Result: $result")
    }

    /// Send all new intents to registered listeners.
//    override fun onNewIntent(intent: Intent) {
//        if (intent.action == Intent.ACTION_SEND) {
//            val payload = intentToJson(intent)
//            val targetUri = intent.getParcelableExtra<Parcelable>(Intent.EXTRA_STREAM).toString()
//            val name = getNameFromUri(activity.applicationContext, Uri.parse(targetUri))
//            if (name != null && name != "") {
//                payload.put("name", name)
//                Log.i("got name", name)
//            }
//            Log.i("triggering event", payload.toString())
//            trigger("share", payload)
//        }
//        helloWorld("Alexis")
//    }
}
