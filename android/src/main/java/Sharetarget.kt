package com.plugin.mobilesharetarget

import android.util.Log

class Sharetarget {
    private val TAG = "mobilesharetarget"

    init {
        try {
            System.loadLibrary(BuildConfig.LIBRARY_NAME)
            Log.d(TAG, "Successfully loaded tauri lib")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Failed to load tauri lib", e)
            throw e
        }
    }

    external fun pushIntent(intent: String)

}
