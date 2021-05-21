//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPresentationAvailabilityListener",
            base: Some("nsISupports"),
            methods: Err("native type const nsTArray<nsString> unsupported"),
        },

        Interface {
            name: "nsIPresentationSessionListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifyStateChange (in AString sessionId, in unsigned short state, in nsresult reason); */
                    Method {
                        name: "NotifyStateChange",
                        params: &[Param { name: "sessionId", ty: "*const ::nsstring::nsAString" }, Param { name: "state", ty: "u16" }, Param { name: "reason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyMessage (in AString sessionId, in ACString data, in boolean isBinary); */
                    Method {
                        name: "NotifyMessage",
                        params: &[Param { name: "sessionId", ty: "*const ::nsstring::nsAString" }, Param { name: "data", ty: "*const ::nsstring::nsACString" }, Param { name: "isBinary", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPresentationRespondingListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifySessionConnect (in unsigned long long windowId, in AString sessionId); */
                    Method {
                        name: "NotifySessionConnect",
                        params: &[Param { name: "windowId", ty: "u64" }, Param { name: "sessionId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

