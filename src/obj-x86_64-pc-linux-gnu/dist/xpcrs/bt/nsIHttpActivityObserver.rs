//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpActivityObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpActivityObserver",
            base: Some("nsISupports"),
            methods: Err("native type const mozilla::net::HttpActivityArgs unsupported"),
        },

        Interface {
            name: "nsIHttpActivityDistributor",
            base: Some("nsIHttpActivityObserver"),
            methods: Ok(&[
                    /* void addObserver (in nsIHttpActivityObserver aObserver); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIHttpActivityObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsIHttpActivityObserver aObserver); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIHttpActivityObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

