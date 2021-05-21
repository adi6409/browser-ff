//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/nsIXPConnect.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXPConnectJSObjectHolder",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIXPConnectWrappedNative",
            base: Some("nsIXPConnectJSObjectHolder"),
            methods: Ok(&[
                    /* void debugDump (in short depth); */
                    Method {
                        name: "DebugDump",
                        params: &[Param { name: "depth", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIXPConnectWrappedJS",
            base: Some("nsIXPConnectJSObjectHolder"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIXPConnectWrappedJSUnmarkGray",
            base: Some("nsIXPConnectWrappedJS"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsIXPConnect",
            base: Some("nsISupports"),
            methods: Err("native type JSContext unsupported"),
        },

        ]; D}

