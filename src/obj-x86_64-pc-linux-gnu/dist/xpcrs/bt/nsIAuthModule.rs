//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthModule",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aServiceName", ty: "*const libc::c_char" }, Param { name: "aServiceFlags", ty: "u32" }, Param { name: "aDomain", ty: "*const i16" }, Param { name: "aUsername", ty: "*const i16" }, Param { name: "aPassword", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "GetNextToken",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "u32" }, Param { name: "aOutToken", ty: "*mut *mut libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "Wrap",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "u32" }, Param { name: "confidential", ty: "bool" }, Param { name: "aOutToken", ty: "*mut *mut libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
                    Method {
                        name: "Unwrap",
                        params: &[Param { name: "aInToken", ty: "*const libc::c_void" }, Param { name: "aInTokenLength", ty: "u32" }, Param { name: "aOutToken", ty: "*mut *mut libc::c_void" }, Param { name: "aOutTokenLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

