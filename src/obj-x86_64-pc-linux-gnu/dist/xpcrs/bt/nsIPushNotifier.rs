//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushNotifier.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushNotifier",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifyPush (in ACString scope, in nsIPrincipal principal, in AString messageId); */
                    Method {
                        name: "NotifyPush",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsACString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "messageId", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in AString messageId, in Array<uint8_t> data); */
                    Method {
                        name: "NotifyPushWithData",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsACString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "messageId", ty: "*const ::nsstring::nsAString" }, Param { name: "data", ty: "*const thin_vec::ThinVec<uint8_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal); */
                    Method {
                        name: "NotifySubscriptionChange",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsACString" }, Param { name: "principal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal); */
                    Method {
                        name: "NotifySubscriptionModified",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsACString" }, Param { name: "principal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notifyError (in ACString scope, in nsIPrincipal principal, in AString message, in uint32_t flags); */
                    Method {
                        name: "NotifyError",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsACString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "message", ty: "*const ::nsstring::nsAString" }, Param { name: "flags", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPushData",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        Interface {
            name: "nsIPushMessage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPushData data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut *const nsIPushData" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

