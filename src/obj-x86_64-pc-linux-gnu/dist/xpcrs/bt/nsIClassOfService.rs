//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassOfService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClassOfService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute unsigned long classFlags; */
                    Method {
                        name: "GetClassFlags",
                        params: &[Param { name: "aClassFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetClassFlags",
                        params: &[Param { name: "aClassFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearClassFlags (in unsigned long flags); */
                    Method {
                        name: "ClearClassFlags",
                        params: &[Param { name: "flags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addClassFlags (in unsigned long flags); */
                    Method {
                        name: "AddClassFlags",
                        params: &[Param { name: "flags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

