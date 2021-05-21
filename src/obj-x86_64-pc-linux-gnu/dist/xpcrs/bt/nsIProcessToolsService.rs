//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/processtools/nsIProcessToolsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProcessToolsService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void kill (in unsigned long long pid); */
                    Method {
                        name: "Kill",
                        params: &[Param { name: "pid", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long long pid; */
                    Method {
                        name: "GetPid",
                        params: &[Param { name: "aPid", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

