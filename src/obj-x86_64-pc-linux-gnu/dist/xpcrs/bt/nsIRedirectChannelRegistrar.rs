//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectChannelRegistrar.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRedirectChannelRegistrar",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerChannel (in nsIChannel channel, in uint64_t id); */
                    Method {
                        name: "RegisterChannel",
                        params: &[Param { name: "channel", ty: "*const nsIChannel" }, Param { name: "id", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIChannel linkChannels (in uint64_t id, in nsIParentChannel channel); */
                    Method {
                        name: "LinkChannels",
                        params: &[Param { name: "id", ty: "uint64_t" }, Param { name: "channel", ty: "*const nsIParentChannel" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIChannel getRegisteredChannel (in uint64_t id); */
                    Method {
                        name: "GetRegisteredChannel",
                        params: &[Param { name: "id", ty: "uint64_t" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIParentChannel getParentChannel (in uint64_t id); */
                    Method {
                        name: "GetParentChannel",
                        params: &[Param { name: "id", ty: "uint64_t" }, Param { name: "_retval", ty: "*mut*const nsIParentChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deregisterChannels (in uint64_t id); */
                    Method {
                        name: "DeregisterChannels",
                        params: &[Param { name: "id", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

