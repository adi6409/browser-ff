//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperties.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProperties",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void get (in string prop, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void set (in string prop, in nsISupports value); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean has (in string prop); */
                    Method {
                        name: "Has",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void undefine (in string prop); */
                    Method {
                        name: "Undefine",
                        params: &[Param { name: "prop", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getKeys (); */
                    Method {
                        name: "GetKeys",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

