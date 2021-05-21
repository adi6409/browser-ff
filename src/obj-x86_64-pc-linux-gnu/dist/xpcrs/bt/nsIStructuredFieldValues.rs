//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/http-sfv/nsIStructuredFieldValues.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISFVBareItem",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVInteger",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute long long value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVString",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute ACString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVBool",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute boolean value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVDecimal",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute double value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVToken",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute ACString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVByteSeq",
            base: Some("nsISFVBareItem"),
            methods: Ok(&[
                    /* attribute ACString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVParams",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISFVBareItem get (in ACString key); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVBareItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void set (in ACString key, in nsISFVBareItem item); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "item", ty: "*const nsISFVBareItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void delete (in ACString key); */
                    Method {
                        name: "Delete",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> keys (); */
                    Method {
                        name: "Keys",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVParametrizable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsISFVParams params; */
                    Method {
                        name: "GetParams",
                        params: &[Param { name: "aParams", ty: "*mut *const nsISFVParams" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVItemOrInnerList",
            base: Some("nsISFVParametrizable"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsISFVSerialize",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString serialize (); */
                    Method {
                        name: "Serialize",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVItem",
            base: Some("nsISFVItemOrInnerList"),
            methods: Ok(&[
                    /* readonly attribute nsISFVBareItem value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut *const nsISFVBareItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString serialize (); */
                    Method {
                        name: "Serialize",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVInnerList",
            base: Some("nsISFVItemOrInnerList"),
            methods: Ok(&[
                    /* attribute Array<nsISFVItem> items; */
                    Method {
                        name: "GetItems",
                        params: &[Param { name: "aItems", ty: "*mut thin_vec::ThinVec<RefPtr<nsISFVItem>>" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetItems",
                        params: &[Param { name: "aItems", ty: "*const thin_vec::ThinVec<RefPtr<nsISFVItem>>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVList",
            base: Some("nsISFVSerialize"),
            methods: Ok(&[
                    /* attribute Array<nsISFVItemOrInnerList> members; */
                    Method {
                        name: "GetMembers",
                        params: &[Param { name: "aMembers", ty: "*mut thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMembers",
                        params: &[Param { name: "aMembers", ty: "*const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseMore (in ACString header); */
                    Method {
                        name: "ParseMore",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVDictionary",
            base: Some("nsISFVSerialize"),
            methods: Ok(&[
                    /* nsISFVItemOrInnerList get (in ACString key); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVItemOrInnerList" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void set (in ACString key, in nsISFVItemOrInnerList member_value); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "member_value", ty: "*const nsISFVItemOrInnerList" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void delete (in ACString key); */
                    Method {
                        name: "Delete",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> keys (); */
                    Method {
                        name: "Keys",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parseMore (in ACString header); */
                    Method {
                        name: "ParseMore",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsISFVService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsISFVDictionary parseDictionary (in ACString header); */
                    Method {
                        name: "ParseDictionary",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVDictionary" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVList parseList (in ACString header); */
                    Method {
                        name: "ParseList",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVList" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVItem parseItem (in ACString header); */
                    Method {
                        name: "ParseItem",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVInteger newInteger (in long long value); */
                    Method {
                        name: "NewInteger",
                        params: &[Param { name: "value", ty: "i64" }, Param { name: "_retval", ty: "*mut *const nsISFVInteger" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVBool newBool (in bool value); */
                    Method {
                        name: "NewBool",
                        params: &[Param { name: "value", ty: "bool" }, Param { name: "_retval", ty: "*mut *const nsISFVBool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVDecimal newDecimal (in double value); */
                    Method {
                        name: "NewDecimal",
                        params: &[Param { name: "value", ty: "libc::c_double" }, Param { name: "_retval", ty: "*mut *const nsISFVDecimal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVString newString (in ACString value); */
                    Method {
                        name: "NewString",
                        params: &[Param { name: "value", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVByteSeq newByteSequence (in ACString value); */
                    Method {
                        name: "NewByteSequence",
                        params: &[Param { name: "value", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVByteSeq" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVToken newToken (in ACString value); */
                    Method {
                        name: "NewToken",
                        params: &[Param { name: "value", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISFVToken" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVParams newParameters (); */
                    Method {
                        name: "NewParameters",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISFVParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVInnerList newInnerList (in Array<nsISFVItem> items, in nsISFVParams params); */
                    Method {
                        name: "NewInnerList",
                        params: &[Param { name: "items", ty: "*const thin_vec::ThinVec<RefPtr<nsISFVItem>>" }, Param { name: "params", ty: "*const nsISFVParams" }, Param { name: "_retval", ty: "*mut *const nsISFVInnerList" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVItem newItem (in nsISFVBareItem value, in nsISFVParams params); */
                    Method {
                        name: "NewItem",
                        params: &[Param { name: "value", ty: "*const nsISFVBareItem" }, Param { name: "params", ty: "*const nsISFVParams" }, Param { name: "_retval", ty: "*mut *const nsISFVItem" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVList newList (in Array<nsISFVItemOrInnerList> members); */
                    Method {
                        name: "NewList",
                        params: &[Param { name: "members", ty: "*const thin_vec::ThinVec<RefPtr<nsISFVItemOrInnerList>>" }, Param { name: "_retval", ty: "*mut *const nsISFVList" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISFVDictionary newDictionary (); */
                    Method {
                        name: "NewDictionary",
                        params: &[Param { name: "_retval", ty: "*mut *const nsISFVDictionary" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

