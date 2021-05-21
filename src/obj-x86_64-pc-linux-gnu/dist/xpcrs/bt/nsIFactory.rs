//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "CreateInstance",
                        params: &[Param { name: "aOuter", ty: "*const nsISupports" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void lockFactory (in boolean lock); */
                    Method {
                        name: "LockFactory",
                        params: &[Param { name: "lock", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

