//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavHistoryService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINavHistoryResultNode",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsINavHistoryContainerResultNode parent; */
                    Method {
                        name: "GetParent",
                        params: &[Param { name: "aParent", ty: "*mut*const nsINavHistoryContainerResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsINavHistoryResult parentResult; */
                    Method {
                        name: "GetParentResult",
                        params: &[Param { name: "aParentResult", ty: "*mut*const nsINavHistoryResult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String uri; */
                    Method {
                        name: "GetUri",
                        params: &[Param { name: "aUri", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String title; */
                    Method {
                        name: "GetTitle",
                        params: &[Param { name: "aTitle", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long accessCount; */
                    Method {
                        name: "GetAccessCount",
                        params: &[Param { name: "aAccessCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime time; */
                    Method {
                        name: "GetTime",
                        params: &[Param { name: "aTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String icon; */
                    Method {
                        name: "GetIcon",
                        params: &[Param { name: "aIcon", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long indentLevel; */
                    Method {
                        name: "GetIndentLevel",
                        params: &[Param { name: "aIndentLevel", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long bookmarkIndex; */
                    Method {
                        name: "GetBookmarkIndex",
                        params: &[Param { name: "aBookmarkIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long itemId; */
                    Method {
                        name: "GetItemId",
                        params: &[Param { name: "aItemId", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime dateAdded; */
                    Method {
                        name: "GetDateAdded",
                        params: &[Param { name: "aDateAdded", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime lastModified; */
                    Method {
                        name: "GetLastModified",
                        params: &[Param { name: "aLastModified", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString tags; */
                    Method {
                        name: "GetTags",
                        params: &[Param { name: "aTags", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString pageGuid; */
                    Method {
                        name: "GetPageGuid",
                        params: &[Param { name: "aPageGuid", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString bookmarkGuid; */
                    Method {
                        name: "GetBookmarkGuid",
                        params: &[Param { name: "aBookmarkGuid", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long visitId; */
                    Method {
                        name: "GetVisitId",
                        params: &[Param { name: "aVisitId", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long fromVisitId; */
                    Method {
                        name: "GetFromVisitId",
                        params: &[Param { name: "aFromVisitId", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long visitType; */
                    Method {
                        name: "GetVisitType",
                        params: &[Param { name: "aVisitType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryContainerResultNode",
            base: Some("nsINavHistoryResultNode"),
            methods: Ok(&[
                    /* attribute boolean containerOpen; */
                    Method {
                        name: "GetContainerOpen",
                        params: &[Param { name: "aContainerOpen", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContainerOpen",
                        params: &[Param { name: "aContainerOpen", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasChildren; */
                    Method {
                        name: "GetHasChildren",
                        params: &[Param { name: "aHasChildren", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long childCount; */
                    Method {
                        name: "GetChildCount",
                        params: &[Param { name: "aChildCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
                    Method {
                        name: "GetChild",
                        params: &[Param { name: "aIndex", ty: "u32" }, Param { name: "_retval", ty: "*mut *const nsINavHistoryResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "GetChildIndex",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryQueryResultNode",
            base: Some("nsINavHistoryContainerResultNode"),
            methods: Ok(&[
                    /* readonly attribute nsINavHistoryQuery query; */
                    Method {
                        name: "GetQuery",
                        params: &[Param { name: "aQuery", ty: "*mut*const nsINavHistoryQuery" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsINavHistoryQueryOptions queryOptions; */
                    Method {
                        name: "GetQueryOptions",
                        params: &[Param { name: "aQueryOptions", ty: "*mut*const nsINavHistoryQueryOptions" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long folderItemId; */
                    Method {
                        name: "GetFolderItemId",
                        params: &[Param { name: "aFolderItemId", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString targetFolderGuid; */
                    Method {
                        name: "GetTargetFolderGuid",
                        params: &[Param { name: "aTargetFolderGuid", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryResultObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean observeHistoryDetails; */
                    Method {
                        name: "GetObserveHistoryDetails",
                        params: &[Param { name: "aObserveHistoryDetails", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
                    Method {
                        name: "NodeInserted",
                        params: &[Param { name: "aParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewIndex", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
                    Method {
                        name: "NodeRemoved",
                        params: &[Param { name: "aParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aItem", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldIndex", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
                    Method {
                        name: "NodeMoved",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aOldIndex", ty: "u32" }, Param { name: "aNewParent", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aNewIndex", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
                    Method {
                        name: "NodeTitleChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewTitle", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aOldURI); */
                    Method {
                        name: "NodeURIChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldURI", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "NodeIconChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aOldVisitDate, in unsigned long aOldAccessCount); */
                    Method {
                        name: "NodeHistoryDetailsChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aOldVisitDate", ty: "PRTime" }, Param { name: "aOldAccessCount", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
                    Method {
                        name: "NodeTagsChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
                    Method {
                        name: "NodeKeywordChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewKeyword", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
                    Method {
                        name: "NodeDateAddedChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewValue", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
                    Method {
                        name: "NodeLastModifiedChanged",
                        params: &[Param { name: "aNode", ty: "*const nsINavHistoryResultNode" }, Param { name: "aNewValue", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
                    Method {
                        name: "ContainerStateChanged",
                        params: &[Param { name: "aContainerNode", ty: "*const nsINavHistoryContainerResultNode" }, Param { name: "aOldState", ty: "u32" }, Param { name: "aNewState", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
                    Method {
                        name: "InvalidateContainer",
                        params: &[Param { name: "aContainerNode", ty: "*const nsINavHistoryContainerResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sortingChanged (in unsigned short sortingMode); */
                    Method {
                        name: "SortingChanged",
                        params: &[Param { name: "sortingMode", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void batching (in boolean aToggleMode); */
                    Method {
                        name: "Batching",
                        params: &[Param { name: "aToggleMode", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsINavHistoryResult result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut*const nsINavHistoryResult" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetResult",
                        params: &[Param { name: "aResult", ty: "*const nsINavHistoryResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryResult",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute unsigned short sortingMode; */
                    Method {
                        name: "GetSortingMode",
                        params: &[Param { name: "aSortingMode", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSortingMode",
                        params: &[Param { name: "aSortingMode", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean suppressNotifications; */
                    Method {
                        name: "GetSuppressNotifications",
                        params: &[Param { name: "aSuppressNotifications", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSuppressNotifications",
                        params: &[Param { name: "aSuppressNotifications", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsINavHistoryResultObserver" }, Param { name: "aOwnsWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsINavHistoryResultObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsINavHistoryContainerResultNode root; */
                    Method {
                        name: "GetRoot",
                        params: &[Param { name: "aRoot", ty: "*mut *const nsINavHistoryContainerResultNode" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onBeginUpdateBatch (); */
                    Method {
                        name: "OnBeginUpdateBatch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onEndUpdateBatch (); */
                    Method {
                        name: "OnEndUpdateBatch",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryQuery",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute PRTime beginTime; */
                    Method {
                        name: "GetBeginTime",
                        params: &[Param { name: "aBeginTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetBeginTime",
                        params: &[Param { name: "aBeginTime", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long beginTimeReference; */
                    Method {
                        name: "GetBeginTimeReference",
                        params: &[Param { name: "aBeginTimeReference", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetBeginTimeReference",
                        params: &[Param { name: "aBeginTimeReference", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasBeginTime; */
                    Method {
                        name: "GetHasBeginTime",
                        params: &[Param { name: "aHasBeginTime", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime absoluteBeginTime; */
                    Method {
                        name: "GetAbsoluteBeginTime",
                        params: &[Param { name: "aAbsoluteBeginTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute PRTime endTime; */
                    Method {
                        name: "GetEndTime",
                        params: &[Param { name: "aEndTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEndTime",
                        params: &[Param { name: "aEndTime", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long endTimeReference; */
                    Method {
                        name: "GetEndTimeReference",
                        params: &[Param { name: "aEndTimeReference", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEndTimeReference",
                        params: &[Param { name: "aEndTimeReference", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasEndTime; */
                    Method {
                        name: "GetHasEndTime",
                        params: &[Param { name: "aHasEndTime", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute PRTime absoluteEndTime; */
                    Method {
                        name: "GetAbsoluteEndTime",
                        params: &[Param { name: "aAbsoluteEndTime", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString searchTerms; */
                    Method {
                        name: "GetSearchTerms",
                        params: &[Param { name: "aSearchTerms", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSearchTerms",
                        params: &[Param { name: "aSearchTerms", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasSearchTerms; */
                    Method {
                        name: "GetHasSearchTerms",
                        params: &[Param { name: "aHasSearchTerms", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long minVisits; */
                    Method {
                        name: "GetMinVisits",
                        params: &[Param { name: "aMinVisits", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMinVisits",
                        params: &[Param { name: "aMinVisits", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long maxVisits; */
                    Method {
                        name: "GetMaxVisits",
                        params: &[Param { name: "aMaxVisits", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxVisits",
                        params: &[Param { name: "aMaxVisits", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTransitions (in Array<unsigned long> transitions); */
                    Method {
                        name: "SetTransitions",
                        params: &[Param { name: "transitions", ty: "*const thin_vec::ThinVec<u32>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<unsigned long> getTransitions (); */
                    Method {
                        name: "GetTransitions",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<u32>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long transitionCount; */
                    Method {
                        name: "GetTransitionCount",
                        params: &[Param { name: "aTransitionCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean onlyBookmarked; */
                    Method {
                        name: "GetOnlyBookmarked",
                        params: &[Param { name: "aOnlyBookmarked", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOnlyBookmarked",
                        params: &[Param { name: "aOnlyBookmarked", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean domainIsHost; */
                    Method {
                        name: "GetDomainIsHost",
                        params: &[Param { name: "aDomainIsHost", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDomainIsHost",
                        params: &[Param { name: "aDomainIsHost", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String domain; */
                    Method {
                        name: "GetDomain",
                        params: &[Param { name: "aDomain", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDomain",
                        params: &[Param { name: "aDomain", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasDomain; */
                    Method {
                        name: "GetHasDomain",
                        params: &[Param { name: "aHasDomain", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURI uri; */
                    Method {
                        name: "GetUri",
                        params: &[Param { name: "aUri", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetUri",
                        params: &[Param { name: "aUri", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasUri; */
                    Method {
                        name: "GetHasUri",
                        params: &[Param { name: "aHasUri", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean annotationIsNot; */
                    Method {
                        name: "GetAnnotationIsNot",
                        params: &[Param { name: "aAnnotationIsNot", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAnnotationIsNot",
                        params: &[Param { name: "aAnnotationIsNot", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AUTF8String annotation; */
                    Method {
                        name: "GetAnnotation",
                        params: &[Param { name: "aAnnotation", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAnnotation",
                        params: &[Param { name: "aAnnotation", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean hasAnnotation; */
                    Method {
                        name: "GetHasAnnotation",
                        params: &[Param { name: "aHasAnnotation", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIVariant tags; */
                    Method {
                        name: "GetTags",
                        params: &[Param { name: "aTags", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTags",
                        params: &[Param { name: "aTags", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean tagsAreNot; */
                    Method {
                        name: "GetTagsAreNot",
                        params: &[Param { name: "aTagsAreNot", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTagsAreNot",
                        params: &[Param { name: "aTagsAreNot", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getParents (); */
                    Method {
                        name: "GetParents",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long parentCount; */
                    Method {
                        name: "GetParentCount",
                        params: &[Param { name: "aParentCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setParents (in Array<ACString> aGuids); */
                    Method {
                        name: "SetParents",
                        params: &[Param { name: "aGuids", ty: "*const thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryQuery clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsINavHistoryQuery" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryQueryOptions",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute unsigned short sortingMode; */
                    Method {
                        name: "GetSortingMode",
                        params: &[Param { name: "aSortingMode", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSortingMode",
                        params: &[Param { name: "aSortingMode", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned short resultType; */
                    Method {
                        name: "GetResultType",
                        params: &[Param { name: "aResultType", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetResultType",
                        params: &[Param { name: "aResultType", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean excludeItems; */
                    Method {
                        name: "GetExcludeItems",
                        params: &[Param { name: "aExcludeItems", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetExcludeItems",
                        params: &[Param { name: "aExcludeItems", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean excludeQueries; */
                    Method {
                        name: "GetExcludeQueries",
                        params: &[Param { name: "aExcludeQueries", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetExcludeQueries",
                        params: &[Param { name: "aExcludeQueries", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean expandQueries; */
                    Method {
                        name: "GetExpandQueries",
                        params: &[Param { name: "aExpandQueries", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetExpandQueries",
                        params: &[Param { name: "aExpandQueries", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean includeHidden; */
                    Method {
                        name: "GetIncludeHidden",
                        params: &[Param { name: "aIncludeHidden", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIncludeHidden",
                        params: &[Param { name: "aIncludeHidden", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long maxResults; */
                    Method {
                        name: "GetMaxResults",
                        params: &[Param { name: "aMaxResults", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxResults",
                        params: &[Param { name: "aMaxResults", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned short queryType; */
                    Method {
                        name: "GetQueryType",
                        params: &[Param { name: "aQueryType", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetQueryType",
                        params: &[Param { name: "aQueryType", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean asyncEnabled; */
                    Method {
                        name: "GetAsyncEnabled",
                        params: &[Param { name: "aAsyncEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAsyncEnabled",
                        params: &[Param { name: "aAsyncEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryQueryOptions clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsINavHistoryQueryOptions" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsINavHistoryService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned short databaseStatus; */
                    Method {
                        name: "GetDatabaseStatus",
                        params: &[Param { name: "aDatabaseStatus", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void markPageAsFollowedBookmark (in nsIURI aURI); */
                    Method {
                        name: "MarkPageAsFollowedBookmark",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void markPageAsTyped (in nsIURI aURI); */
                    Method {
                        name: "MarkPageAsTyped",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void markPageAsFollowedLink (in nsIURI aURI); */
                    Method {
                        name: "MarkPageAsFollowedLink",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canAddURI (in nsIURI aURI); */
                    Method {
                        name: "CanAddURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryQuery getNewQuery (); */
                    Method {
                        name: "GetNewQuery",
                        params: &[Param { name: "_retval", ty: "*mut *const nsINavHistoryQuery" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryQueryOptions getNewQueryOptions (); */
                    Method {
                        name: "GetNewQueryOptions",
                        params: &[Param { name: "_retval", ty: "*mut *const nsINavHistoryQueryOptions" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
                    Method {
                        name: "ExecuteQuery",
                        params: &[Param { name: "aQuery", ty: "*const nsINavHistoryQuery" }, Param { name: "options", ty: "*const nsINavHistoryQueryOptions" }, Param { name: "_retval", ty: "*mut *const nsINavHistoryResult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void queryStringToQuery (in AUTF8String aQueryString, out nsINavHistoryQuery aQuery, out nsINavHistoryQueryOptions options); */
                    Method {
                        name: "QueryStringToQuery",
                        params: &[Param { name: "aQueryString", ty: "*const ::nsstring::nsACString" }, Param { name: "aQuery", ty: "*mut *const nsINavHistoryQuery" }, Param { name: "options", ty: "*mut *const nsINavHistoryQueryOptions" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String queryToQueryString (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
                    Method {
                        name: "QueryToQueryString",
                        params: &[Param { name: "aQuery", ty: "*const nsINavHistoryQuery" }, Param { name: "options", ty: "*const nsINavHistoryQueryOptions" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak); */
                    Method {
                        name: "AddObserver",
                        params: &[Param { name: "observer", ty: "*const nsINavHistoryObserver" }, Param { name: "ownsWeak", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserver (in nsINavHistoryObserver observer); */
                    Method {
                        name: "RemoveObserver",
                        params: &[Param { name: "observer", ty: "*const nsINavHistoryObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsINavHistoryObserver> getObservers (); */
                    Method {
                        name: "GetObservers",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsINavHistoryObserver>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean historyDisabled; */
                    Method {
                        name: "GetHistoryDisabled",
                        params: &[Param { name: "aHistoryDisabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString makeGuid (); */
                    Method {
                        name: "MakeGuid",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long long hashURL (in ACString aSpec, [optional] in ACString aMode); */
                    Method {
                        name: "HashURL",
                        params: &[Param { name: "aSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "aMode", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void recalculateOriginFrecencyStats ([optional] in nsIObserver aCallback); */
                    Method {
                        name: "RecalculateOriginFrecencyStats",
                        params: &[Param { name: "aCallback", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIStorageConnection DBConnection; */
                    Method {
                        name: "GetDBConnection",
                        params: &[Param { name: "aDBConnection", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStoragePendingStatement asyncExecuteLegacyQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback); */
                    Method {
                        name: "AsyncExecuteLegacyQuery",
                        params: &[Param { name: "aQuery", ty: "*const nsINavHistoryQuery" }, Param { name: "aOptions", ty: "*const nsINavHistoryQueryOptions" }, Param { name: "aCallback", ty: "*const mozIStorageStatementCallback" }, Param { name: "_retval", ty: "*mut*const mozIStoragePendingStatement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient shutdownClient; */
                    Method {
                        name: "GetShutdownClient",
                        params: &[Param { name: "aShutdownClient", ty: "*mut*const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAsyncShutdownClient connectionShutdownClient; */
                    Method {
                        name: "GetConnectionShutdownClient",
                        params: &[Param { name: "aConnectionShutdownClient", ty: "*mut*const nsIAsyncShutdownClient" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void decayFrecency (); */
                    Method {
                        name: "DecayFrecency",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

