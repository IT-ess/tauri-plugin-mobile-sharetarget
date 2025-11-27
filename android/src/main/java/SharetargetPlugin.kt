package com.plugin.mobilesharetarget

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Parcelable
import android.provider.OpenableColumns
import android.util.Log
import android.webkit.WebView
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import androidx.core.net.toUri
import androidx.lifecycle.ViewModelProvider.NewInstanceFactory.Companion.instance


@TauriPlugin
class SharetargetPlugin(private val activity: Activity): Plugin(activity) {

    private var webView: WebView? = null
    private val implementation = Sharetarget()

    companion object {
        var instance: SharetargetPlugin? = null
    }

    override fun load(webView: WebView) {
        instance = this

        val intent = activity.intent

        if (intent.action == Intent.ACTION_SEND) {
            val payload = intentToJson(intent)
            val targetUri = intent.getParcelableExtra<Parcelable>(Intent.EXTRA_STREAM).toString()
            val name = getNameFromUri(activity.applicationContext, targetUri.toUri())
            if (name != null && name != "") {
                payload.put("name", name)
                Log.i("got name", name)
            }
            Log.i("triggering event", payload.toString())
            implementation.pushIntent(payload.toString())
        }

        super.load(webView)
        this.webView = webView
    }

    /// Send all new intents to registered listeners.
    override fun onNewIntent(intent: Intent) {
        if (intent.action == Intent.ACTION_SEND) {
            val payload = intentToJson(intent)
            val targetUri = intent.getParcelableExtra<Parcelable>(Intent.EXTRA_STREAM).toString()
            val name = getNameFromUri(activity.applicationContext, targetUri.toUri())
            if (name != null && name != "") {
                payload.put("name", name)
                Log.i("got name", name)
            }
            Log.i("triggering event", payload.toString())
            implementation.pushIntent(payload.toString())
        }
    }
}

fun intentToJson(intent: Intent): JSObject {
    val json = JSObject()
    Log.i("processing", intent.toUri(0))
    json.put("uri", intent.toUri(0))
    json.put("content_type", intent.type)
    val streamUrl = intent.extras?.get("android.intent.extra.STREAM")
    if (streamUrl != null) {
        json.put("stream", streamUrl)
    }
    /*
        }
    }
    */
    return json
}
fun getNameFromUri(context: Context, uri: Uri): String? {
    var displayName: String? = ""
    val projection = arrayOf(OpenableColumns.DISPLAY_NAME)
    val cursor =
        context.contentResolver.query(uri, projection, null, null, null)
    if (cursor != null) {
        cursor.moveToFirst()
        val columnIdx = cursor.getColumnIndex(projection[0])
        displayName = cursor.getString(columnIdx)
        cursor.close()
    }
    if (displayName.isNullOrEmpty()) {
        displayName = uri.lastPathSegment
    }
    return displayName
}