package com.plugin.mobilesharetarget

import android.util.Log

class Sharetarget {
    private val TAG = "mobilesharetarget"

    init {
        try {
            System.loadLibrary(BuildConfig.TAURI_LIBRARY_NAME)
            Log.d(TAG, "Successfully loaded ${BuildConfig.TAURI_LIBRARY_NAME}")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Could not load native library", e)
            throw e
        }
    }

    external fun pushIntent(intent: String)

}
