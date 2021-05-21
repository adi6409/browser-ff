//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIWeakReference.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWeakReference",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [binaryname(QueryReferentFromScript)] void QueryReferent (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
                    Method {
                        name: "QueryReferentFromScript",
                        params: &[Param { name: "uuid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISupportsWeakReference",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIWeakReference GetWeakReference (); */
                    Method {
                        name: "GetWeakReference",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIWeakReference" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

