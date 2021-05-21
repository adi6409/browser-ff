//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPushSubscription",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIPushSubscriptionCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
                    Method {
                        name: "OnPushSubscription",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "subscription", ty: "*const nsIPushSubscription" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUnsubscribeResultCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onUnsubscribe (in nsresult status, in bool success); */
                    Method {
                        name: "OnUnsubscribe",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "success", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPushClearResultCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onClear (in nsresult status); */
                    Method {
                        name: "OnClear",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPushService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString pushTopic; */
                    Method {
                        name: "GetPushTopic",
                        params: &[Param { name: "aPushTopic", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString subscriptionChangeTopic; */
                    Method {
                        name: "GetSubscriptionChangeTopic",
                        params: &[Param { name: "aSubscriptionChangeTopic", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString subscriptionModifiedTopic; */
                    Method {
                        name: "GetSubscriptionModifiedTopic",
                        params: &[Param { name: "aSubscriptionModifiedTopic", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void subscribe (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
                    Method {
                        name: "Subscribe",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsAString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "callback", ty: "*const nsIPushSubscriptionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void subscribeWithKey (in AString scope, in nsIPrincipal principal, in Array<uint8_t> key, in nsIPushSubscriptionCallback callback); */
                    Method {
                        name: "SubscribeWithKey",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsAString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "key", ty: "*const thin_vec::ThinVec<uint8_t>" }, Param { name: "callback", ty: "*const nsIPushSubscriptionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unsubscribe (in AString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback); */
                    Method {
                        name: "Unsubscribe",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsAString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "callback", ty: "*const nsIUnsubscribeResultCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getSubscription (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
                    Method {
                        name: "GetSubscription",
                        params: &[Param { name: "scope", ty: "*const ::nsstring::nsAString" }, Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "callback", ty: "*const nsIPushSubscriptionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearForDomain (in AString domain, in nsIPushClearResultCallback callback); */
                    Method {
                        name: "ClearForDomain",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "callback", ty: "*const nsIPushClearResultCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPushQuotaManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notificationForOriginShown (in string origin); */
                    Method {
                        name: "NotificationForOriginShown",
                        params: &[Param { name: "origin", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void notificationForOriginClosed (in string origin); */
                    Method {
                        name: "NotificationForOriginClosed",
                        params: &[Param { name: "origin", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

