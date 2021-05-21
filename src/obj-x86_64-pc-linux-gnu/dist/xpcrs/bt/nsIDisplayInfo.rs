//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIDisplayInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDisplayInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean connected; */
                    Method {
                        name: "GetConnected",
                        params: &[Param { name: "aConnected", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

