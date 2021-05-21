//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPrivateBrowsingChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPrivateBrowsingChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setPrivate (in boolean aPrivate); */
                    Method {
                        name: "SetPrivate",
                        params: &[Param { name: "aPrivate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isChannelPrivate; */
                    Method {
                        name: "GetIsChannelPrivate",
                        params: &[Param { name: "aIsChannelPrivate", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
                    Method {
                        name: "IsPrivateModeOverriden",
                        params: &[Param { name: "aValue", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

