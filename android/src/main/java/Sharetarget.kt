package com.plugin.mobilesharetarget

import android.util.Log

class Sharetarget {
    private val TAG = "mobilesharetarget"

    init {
        try {
            System.loadLibrary("tauri_app_lib")
            Log.d(TAG, "Successfully loaded libtauri_app_lib.so")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Faileapp_libd to load libtauri_app_lib.so", e)
            throw e
        }
    }

    external fun pushIntent(intent: String)

}
