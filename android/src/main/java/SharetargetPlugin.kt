package com.plugin.mobilesharetarget

import android.app.Activity
import android.content.Intent
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.annotation.InvokeArg


@InvokeArg
class Config {
    var lib: String? = "tauri_app_lib"
}

@TauriPlugin
class SharetargetPlugin(private val activity: Activity): Plugin(activity) {
    private var lib: String? = "tauri_app_lib"
    private var implementation = Sharetarget(lib)

    /// Handle intents when app is being launched
    override fun load(webView: WebView) {
        getConfig(Config::class.java).let {
            this.lib = it.lib
        }

        this.implementation = Sharetarget(lib)

        val intent = activity.intent

        if (intent.action == Intent.ACTION_SEND) {
            implementation.pushIntent(intent.toUri(0))
        }
    }

    /// Handle intents when app is already launched
    override fun onNewIntent(intent: Intent) {
        if (intent.action == Intent.ACTION_SEND) {
            implementation.pushIntent(intent.toUri(0))
        }
    }
}
