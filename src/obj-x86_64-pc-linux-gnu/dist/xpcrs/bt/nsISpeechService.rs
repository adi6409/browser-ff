//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISpeechService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISpeechTaskCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPause (); */
                    Method {
                        name: "OnPause",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onResume (); */
                    Method {
                        name: "OnResume",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCancel (); */
                    Method {
                        name: "OnCancel",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onVolumeChanged (in float aVolume); */
                    Method {
                        name: "OnVolumeChanged",
                        params: &[Param { name: "aVolume", ty: "libc::c_float" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISpeechTask",
            base: Some("nsISupports"),
            methods: Err("optional_argc is unsupported"),
        },

        Interface {
            name: "nsISpeechService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void speak (in AString aText, in AString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
                    Method {
                        name: "Speak",
                        params: &[Param { name: "aText", ty: "*const ::nsstring::nsAString" }, Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "aVolume", ty: "libc::c_float" }, Param { name: "aRate", ty: "libc::c_float" }, Param { name: "aPitch", ty: "libc::c_float" }, Param { name: "aTask", ty: "*const nsISpeechTask" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

