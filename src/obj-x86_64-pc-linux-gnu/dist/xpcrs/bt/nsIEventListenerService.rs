//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/events/nsIEventListenerService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEventListenerChange",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute EventTarget target; */
                    Method {
                        name: "GetTarget",
                        params: &[Param { name: "aTarget", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] readonly attribute uint32_t countOfEventListenerChangesAffectingAccessibility; */
                    Method {
                        name: "GetCountOfEventListenerChangesAffectingAccessibility",
                        params: &[Param { name: "aCountOfEventListenerChangesAffectingAccessibility", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIListenerChangeListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void listenersChanged (in nsIArray aEventListenerChanges); */
                    Method {
                        name: "ListenersChanged",
                        params: &[Param { name: "aEventListenerChanges", ty: "*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIEventListenerInfo",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIEventListenerService",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        ]; D}

