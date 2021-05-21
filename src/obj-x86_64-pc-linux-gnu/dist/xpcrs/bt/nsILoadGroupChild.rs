//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadGroupChild.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoadGroupChild",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsILoadGroup parentLoadGroup; */
                    Method {
                        name: "GetParentLoadGroup",
                        params: &[Param { name: "aParentLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetParentLoadGroup",
                        params: &[Param { name: "aParentLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsILoadGroup childLoadGroup; */
                    Method {
                        name: "GetChildLoadGroup",
                        params: &[Param { name: "aChildLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsILoadGroup rootLoadGroup; */
                    Method {
                        name: "GetRootLoadGroup",
                        params: &[Param { name: "aRootLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

