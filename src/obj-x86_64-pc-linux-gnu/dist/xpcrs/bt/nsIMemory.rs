//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMemory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void heapMinimize (in boolean immediate); */
                    Method {
                        name: "HeapMinimize",
                        params: &[Param { name: "immediate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isLowMemoryPlatform (); */
                    Method {
                        name: "IsLowMemoryPlatform",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

