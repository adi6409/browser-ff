//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIDeviceSensors.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDeviceSensorData",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double x; */
                    Method {
                        name: "GetX",
                        params: &[Param { name: "aX", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double y; */
                    Method {
                        name: "GetY",
                        params: &[Param { name: "aY", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double z; */
                    Method {
                        name: "GetZ",
                        params: &[Param { name: "aZ", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDeviceSensors",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "HasWindowListener",
                        params: &[Param { name: "aType", ty: "u32" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "AddWindowListener",
                        params: &[Param { name: "aType", ty: "u32" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
                    Method {
                        name: "RemoveWindowListener",
                        params: &[Param { name: "aType", ty: "u32" }, Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
                    Method {
                        name: "RemoveWindowAsListener",
                        params: &[Param { name: "aWindow", ty: "*const nsIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

