//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/audiochannel/nsIAudioChannelAgent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISuspendedTypes",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsIAudioChannelAgentCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
                    Method {
                        name: "WindowVolumeChanged",
                        params: &[Param { name: "aVolume", ty: "libc::c_float" }, Param { name: "aMuted", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void windowSuspendChanged (in uint32_t aSuspend); */
                    Method {
                        name: "WindowSuspendChanged",
                        params: &[Param { name: "aSuspend", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void windowAudioCaptureChanged (in bool aCapture); */
                    Method {
                        name: "WindowAudioCaptureChanged",
                        params: &[Param { name: "aCapture", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAudioChannelAgent",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "callback", ty: "*const nsIAudioChannelAgentCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initWithWeakCallback (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
                    Method {
                        name: "InitWithWeakCallback",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "callback", ty: "*const nsIAudioChannelAgentCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyStartedPlaying (in uint8_t audible); */
                    Method {
                        name: "NotifyStartedPlaying",
                        params: &[Param { name: "audible", ty: "uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyStoppedPlaying (); */
                    Method {
                        name: "NotifyStoppedPlaying",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyStartedAudible (in uint8_t audible, in uint32_t reason); */
                    Method {
                        name: "NotifyStartedAudible",
                        params: &[Param { name: "audible", ty: "uint8_t" }, Param { name: "reason", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

