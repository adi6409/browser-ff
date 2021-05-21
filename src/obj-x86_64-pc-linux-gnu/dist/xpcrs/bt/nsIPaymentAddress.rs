//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentAddress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPaymentAddress",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString country; */
                    Method {
                        name: "GetCountry",
                        params: &[Param { name: "aCountry", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray addressLine; */
                    Method {
                        name: "GetAddressLine",
                        params: &[Param { name: "aAddressLine", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString region; */
                    Method {
                        name: "GetRegion",
                        params: &[Param { name: "aRegion", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString regionCode; */
                    Method {
                        name: "GetRegionCode",
                        params: &[Param { name: "aRegionCode", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString city; */
                    Method {
                        name: "GetCity",
                        params: &[Param { name: "aCity", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString dependentLocality; */
                    Method {
                        name: "GetDependentLocality",
                        params: &[Param { name: "aDependentLocality", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString postalCode; */
                    Method {
                        name: "GetPostalCode",
                        params: &[Param { name: "aPostalCode", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString sortingCode; */
                    Method {
                        name: "GetSortingCode",
                        params: &[Param { name: "aSortingCode", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString organization; */
                    Method {
                        name: "GetOrganization",
                        params: &[Param { name: "aOrganization", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString recipient; */
                    Method {
                        name: "GetRecipient",
                        params: &[Param { name: "aRecipient", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString phone; */
                    Method {
                        name: "GetPhone",
                        params: &[Param { name: "aPhone", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aCountry, in nsIArray aAddressLine, in AString aRegion, in AString aRegionCode, in AString aCity, in AString aDependentLocality, in AString aPostalCode, in AString aSortingCode, in AString aOrganization, in AString aRecipient, in AString aPhone); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aCountry", ty: "*const ::nsstring::nsAString" }, Param { name: "aAddressLine", ty: "*const nsIArray" }, Param { name: "aRegion", ty: "*const ::nsstring::nsAString" }, Param { name: "aRegionCode", ty: "*const ::nsstring::nsAString" }, Param { name: "aCity", ty: "*const ::nsstring::nsAString" }, Param { name: "aDependentLocality", ty: "*const ::nsstring::nsAString" }, Param { name: "aPostalCode", ty: "*const ::nsstring::nsAString" }, Param { name: "aSortingCode", ty: "*const ::nsstring::nsAString" }, Param { name: "aOrganization", ty: "*const ::nsstring::nsAString" }, Param { name: "aRecipient", ty: "*const ::nsstring::nsAString" }, Param { name: "aPhone", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

