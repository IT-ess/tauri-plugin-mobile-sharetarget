package com.plugin.mobilesharetarget

import android.util.Log

class Example {
    private val TAG = "mobilesharetarget"

    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }


    init {
        try {
            System.loadLibrary("tauri_app_lib")
            Log.d(TAG, "Successfully loaded libtauri_app_lib.so")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Faileapp_libd to load libtauri_app_lib.so", e)
            throw e
        }
    }

    external fun helloWorld(name: String): String?
}
