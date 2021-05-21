//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISynthVoiceRegistry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISynthVoiceRegistry",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addVoice (in nsISpeechService aService, in AString aUri, in AString aName, in AString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
                    Method {
                        name: "AddVoice",
                        params: &[Param { name: "aService", ty: "*const nsISpeechService" }, Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aLang", ty: "*const ::nsstring::nsAString" }, Param { name: "aLocalService", ty: "bool" }, Param { name: "aQueuesUtterances", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeVoice (in nsISpeechService aService, in AString aUri); */
                    Method {
                        name: "RemoveVoice",
                        params: &[Param { name: "aService", ty: "*const nsISpeechService" }, Param { name: "aUri", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyVoicesChanged (); */
                    Method {
                        name: "NotifyVoicesChanged",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDefaultVoice (in AString aUri, in boolean aIsDefault); */
                    Method {
                        name: "SetDefaultVoice",
                        params: &[Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "aIsDefault", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t voiceCount; */
                    Method {
                        name: "GetVoiceCount",
                        params: &[Param { name: "aVoiceCount", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getVoice (in uint32_t aIndex); */
                    Method {
                        name: "GetVoice",
                        params: &[Param { name: "aIndex", ty: "uint32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isDefaultVoice (in AString aUri); */
                    Method {
                        name: "IsDefaultVoice",
                        params: &[Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool isLocalVoice (in AString aUri); */
                    Method {
                        name: "IsLocalVoice",
                        params: &[Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getVoiceLang (in AString aUri); */
                    Method {
                        name: "GetVoiceLang",
                        params: &[Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getVoiceName (in AString aUri); */
                    Method {
                        name: "GetVoiceName",
                        params: &[Param { name: "aUri", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

