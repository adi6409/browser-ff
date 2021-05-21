/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessiblePivot.idl
 */

#ifndef __gen_nsIAccessiblePivot_h__
#define __gen_nsIAccessiblePivot_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
typedef int16_t  TextBoundaryType;

typedef int16_t  PivotMoveReason;

class nsIAccessible; /* forward declaration */

class nsIAccessibleText; /* forward declaration */

class nsIAccessibleTraversalRule; /* forward declaration */

class nsIAccessiblePivotObserver; /* forward declaration */


/* starting interface:    nsIAccessiblePivot */
#define NS_IACCESSIBLEPIVOT_IID_STR "81fe5144-059b-42db-bd3a-f6ce3158d5e9"

#define NS_IACCESSIBLEPIVOT_IID \
  {0x81fe5144, 0x059b, 0x42db, \
    { 0xbd, 0x3a, 0xf6, 0xce, 0x31, 0x58, 0xd5, 0xe9 }}

class NS_NO_VTABLE nsIAccessiblePivot : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEPIVOT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessiblePivot;

  enum {
    NO_BOUNDARY = -1,
    CHAR_BOUNDARY = 0,
    WORD_BOUNDARY = 1,
    LINE_BOUNDARY = 2,
    ATTRIBUTE_RANGE_BOUNDARY = 3,
    REASON_NONE = 0,
    REASON_NEXT = 1,
    REASON_PREV = 2,
    REASON_FIRST = 3,
    REASON_LAST = 4,
    REASON_POINT = 5
  };

  /* attribute nsIAccessible position; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPosition(nsIAccessible **aPosition) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPosition(nsIAccessible *aPosition) = 0;

  /* readonly attribute nsIAccessible root; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRoot(nsIAccessible **aRoot) = 0;

  /* attribute nsIAccessible modalRoot; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetModalRoot(nsIAccessible **aModalRoot) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetModalRoot(nsIAccessible *aModalRoot) = 0;

  /* readonly attribute long startOffset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStartOffset(int32_t *aStartOffset) = 0;

  /* readonly attribute long endOffset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEndOffset(int32_t *aEndOffset) = 0;

  /* [optional_argc] void setTextRange (in nsIAccessibleText aTextAccessible, in long aStartOffset, in long aEndOffset, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTextRange(nsIAccessibleText *aTextAccessible, int32_t aStartOffset, int32_t aEndOffset, bool aIsFromUserInput, uint8_t _argc) = 0;

  /* [optional_argc] boolean moveNext (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveNext(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean movePrevious (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MovePrevious(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean moveFirst (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveFirst(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean moveLast (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveLast(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean moveNextByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveNextByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean movePreviousByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MovePreviousByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* [optional_argc] boolean moveToPoint (in nsIAccessibleTraversalRule aRule, in long aX, in long aY, in boolean aIgnoreNoMatch, [optional] in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveToPoint(nsIAccessibleTraversalRule *aRule, int32_t aX, int32_t aY, bool aIgnoreNoMatch, bool aIsFromUserInput, uint8_t _argc, bool *_retval) = 0;

  /* void addObserver (in nsIAccessiblePivotObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddObserver(nsIAccessiblePivotObserver *aObserver) = 0;

  /* void removeObserver (in nsIAccessiblePivotObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveObserver(nsIAccessiblePivotObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessiblePivot, NS_IACCESSIBLEPIVOT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEPIVOT \
  NS_IMETHOD GetPosition(nsIAccessible **aPosition) override; \
  NS_IMETHOD SetPosition(nsIAccessible *aPosition) override; \
  NS_IMETHOD GetRoot(nsIAccessible **aRoot) override; \
  NS_IMETHOD GetModalRoot(nsIAccessible **aModalRoot) override; \
  NS_IMETHOD SetModalRoot(nsIAccessible *aModalRoot) override; \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override; \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override; \
  NS_IMETHOD SetTextRange(nsIAccessibleText *aTextAccessible, int32_t aStartOffset, int32_t aEndOffset, bool aIsFromUserInput, uint8_t _argc) override; \
  NS_IMETHOD MoveNext(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MovePrevious(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MoveFirst(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MoveLast(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MoveNextByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MovePreviousByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD MoveToPoint(nsIAccessibleTraversalRule *aRule, int32_t aX, int32_t aY, bool aIgnoreNoMatch, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD AddObserver(nsIAccessiblePivotObserver *aObserver) override; \
  NS_IMETHOD RemoveObserver(nsIAccessiblePivotObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEPIVOT \
  nsresult GetPosition(nsIAccessible **aPosition); \
  nsresult SetPosition(nsIAccessible *aPosition); \
  nsresult GetRoot(nsIAccessible **aRoot); \
  nsresult GetModalRoot(nsIAccessible **aModalRoot); \
  nsresult SetModalRoot(nsIAccessible *aModalRoot); \
  nsresult GetStartOffset(int32_t *aStartOffset); \
  nsresult GetEndOffset(int32_t *aEndOffset); \
  nsresult SetTextRange(nsIAccessibleText *aTextAccessible, int32_t aStartOffset, int32_t aEndOffset, bool aIsFromUserInput, uint8_t _argc); \
  nsresult MoveNext(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MovePrevious(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MoveFirst(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MoveLast(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MoveNextByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MovePreviousByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult MoveToPoint(nsIAccessibleTraversalRule *aRule, int32_t aX, int32_t aY, bool aIgnoreNoMatch, bool aIsFromUserInput, uint8_t _argc, bool *_retval); \
  nsresult AddObserver(nsIAccessiblePivotObserver *aObserver); \
  nsresult RemoveObserver(nsIAccessiblePivotObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEPIVOT(_to) \
  NS_IMETHOD GetPosition(nsIAccessible **aPosition) override { return _to GetPosition(aPosition); } \
  NS_IMETHOD SetPosition(nsIAccessible *aPosition) override { return _to SetPosition(aPosition); } \
  NS_IMETHOD GetRoot(nsIAccessible **aRoot) override { return _to GetRoot(aRoot); } \
  NS_IMETHOD GetModalRoot(nsIAccessible **aModalRoot) override { return _to GetModalRoot(aModalRoot); } \
  NS_IMETHOD SetModalRoot(nsIAccessible *aModalRoot) override { return _to SetModalRoot(aModalRoot); } \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override { return _to GetStartOffset(aStartOffset); } \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override { return _to GetEndOffset(aEndOffset); } \
  NS_IMETHOD SetTextRange(nsIAccessibleText *aTextAccessible, int32_t aStartOffset, int32_t aEndOffset, bool aIsFromUserInput, uint8_t _argc) override { return _to SetTextRange(aTextAccessible, aStartOffset, aEndOffset, aIsFromUserInput, _argc); } \
  NS_IMETHOD MoveNext(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MoveNext(aRule, aAnchor, aIncludeStart, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MovePrevious(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MovePrevious(aRule, aAnchor, aIncludeStart, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveFirst(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MoveFirst(aRule, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveLast(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MoveLast(aRule, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveNextByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MoveNextByText(aBoundary, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MovePreviousByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MovePreviousByText(aBoundary, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveToPoint(nsIAccessibleTraversalRule *aRule, int32_t aX, int32_t aY, bool aIgnoreNoMatch, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return _to MoveToPoint(aRule, aX, aY, aIgnoreNoMatch, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD AddObserver(nsIAccessiblePivotObserver *aObserver) override { return _to AddObserver(aObserver); } \
  NS_IMETHOD RemoveObserver(nsIAccessiblePivotObserver *aObserver) override { return _to RemoveObserver(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEPIVOT(_to) \
  NS_IMETHOD GetPosition(nsIAccessible **aPosition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPosition(aPosition); } \
  NS_IMETHOD SetPosition(nsIAccessible *aPosition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPosition(aPosition); } \
  NS_IMETHOD GetRoot(nsIAccessible **aRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRoot(aRoot); } \
  NS_IMETHOD GetModalRoot(nsIAccessible **aModalRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModalRoot(aModalRoot); } \
  NS_IMETHOD SetModalRoot(nsIAccessible *aModalRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetModalRoot(aModalRoot); } \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartOffset(aStartOffset); } \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndOffset(aEndOffset); } \
  NS_IMETHOD SetTextRange(nsIAccessibleText *aTextAccessible, int32_t aStartOffset, int32_t aEndOffset, bool aIsFromUserInput, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTextRange(aTextAccessible, aStartOffset, aEndOffset, aIsFromUserInput, _argc); } \
  NS_IMETHOD MoveNext(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveNext(aRule, aAnchor, aIncludeStart, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MovePrevious(nsIAccessibleTraversalRule *aRule, nsIAccessible *aAnchor, bool aIncludeStart, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MovePrevious(aRule, aAnchor, aIncludeStart, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveFirst(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveFirst(aRule, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveLast(nsIAccessibleTraversalRule *aRule, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveLast(aRule, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveNextByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveNextByText(aBoundary, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MovePreviousByText(TextBoundaryType aBoundary, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MovePreviousByText(aBoundary, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD MoveToPoint(nsIAccessibleTraversalRule *aRule, int32_t aX, int32_t aY, bool aIgnoreNoMatch, bool aIsFromUserInput, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveToPoint(aRule, aX, aY, aIgnoreNoMatch, aIsFromUserInput, _argc, _retval); } \
  NS_IMETHOD AddObserver(nsIAccessiblePivotObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddObserver(aObserver); } \
  NS_IMETHOD RemoveObserver(nsIAccessiblePivotObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveObserver(aObserver); } 


/* starting interface:    nsIAccessiblePivotObserver */
#define NS_IACCESSIBLEPIVOTOBSERVER_IID_STR "6006e502-3861-49bd-aba1-fa6d2e74e237"

#define NS_IACCESSIBLEPIVOTOBSERVER_IID \
  {0x6006e502, 0x3861, 0x49bd, \
    { 0xab, 0xa1, 0xfa, 0x6d, 0x2e, 0x74, 0xe2, 0x37 }}

class NS_NO_VTABLE nsIAccessiblePivotObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEPIVOTOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessiblePivotObserver;

  /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in nsIAccessible aNewAccessible, in long aNewStart, in long aNewEnd, in PivotMoveReason aReason, in TextBoundaryType aBoundaryType, in boolean aIsFromUserInput); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnPivotChanged(nsIAccessiblePivot *aPivot, nsIAccessible *aOldAccessible, int32_t aOldStart, int32_t aOldEnd, nsIAccessible *aNewAccessible, int32_t aNewStart, int32_t aNewEnd, PivotMoveReason aReason, TextBoundaryType aBoundaryType, bool aIsFromUserInput) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessiblePivotObserver, NS_IACCESSIBLEPIVOTOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEPIVOTOBSERVER \
  NS_IMETHOD OnPivotChanged(nsIAccessiblePivot *aPivot, nsIAccessible *aOldAccessible, int32_t aOldStart, int32_t aOldEnd, nsIAccessible *aNewAccessible, int32_t aNewStart, int32_t aNewEnd, PivotMoveReason aReason, TextBoundaryType aBoundaryType, bool aIsFromUserInput) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEPIVOTOBSERVER \
  nsresult OnPivotChanged(nsIAccessiblePivot *aPivot, nsIAccessible *aOldAccessible, int32_t aOldStart, int32_t aOldEnd, nsIAccessible *aNewAccessible, int32_t aNewStart, int32_t aNewEnd, PivotMoveReason aReason, TextBoundaryType aBoundaryType, bool aIsFromUserInput); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEPIVOTOBSERVER(_to) \
  NS_IMETHOD OnPivotChanged(nsIAccessiblePivot *aPivot, nsIAccessible *aOldAccessible, int32_t aOldStart, int32_t aOldEnd, nsIAccessible *aNewAccessible, int32_t aNewStart, int32_t aNewEnd, PivotMoveReason aReason, TextBoundaryType aBoundaryType, bool aIsFromUserInput) override { return _to OnPivotChanged(aPivot, aOldAccessible, aOldStart, aOldEnd, aNewAccessible, aNewStart, aNewEnd, aReason, aBoundaryType, aIsFromUserInput); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEPIVOTOBSERVER(_to) \
  NS_IMETHOD OnPivotChanged(nsIAccessiblePivot *aPivot, nsIAccessible *aOldAccessible, int32_t aOldStart, int32_t aOldEnd, nsIAccessible *aNewAccessible, int32_t aNewStart, int32_t aNewEnd, PivotMoveReason aReason, TextBoundaryType aBoundaryType, bool aIsFromUserInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnPivotChanged(aPivot, aOldAccessible, aOldStart, aOldEnd, aNewAccessible, aNewStart, aNewEnd, aReason, aBoundaryType, aIsFromUserInput); } 


/* starting interface:    nsIAccessibleTraversalRule */
#define NS_IACCESSIBLETRAVERSALRULE_IID_STR "e197460d-1eff-4247-b4bb-a43be1840dae"

#define NS_IACCESSIBLETRAVERSALRULE_IID \
  {0xe197460d, 0x1eff, 0x4247, \
    { 0xb4, 0xbb, 0xa4, 0x3b, 0xe1, 0x84, 0x0d, 0xae }}

class NS_NO_VTABLE nsIAccessibleTraversalRule : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETRAVERSALRULE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTraversalRule;

  enum {
    FILTER_IGNORE = 0U,
    FILTER_MATCH = 1U,
    FILTER_IGNORE_SUBTREE = 2U,
    PREFILTER_INVISIBLE = 1U,
    PREFILTER_OFFSCREEN = 2U,
    PREFILTER_NOT_FOCUSABLE = 4U,
    PREFILTER_TRANSPARENT = 8U,
    PREFILTER_PLATFORM_PRUNED = 16U
  };

  /* readonly attribute unsigned long preFilter; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPreFilter(uint32_t *aPreFilter) = 0;

  /* Array<unsigned long> getMatchRoles (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchRoles(nsTArray<uint32_t >& _retval) = 0;

  /* unsigned short match (in nsIAccessible aAccessible); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Match(nsIAccessible *aAccessible, uint16_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTraversalRule, NS_IACCESSIBLETRAVERSALRULE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETRAVERSALRULE \
  NS_IMETHOD GetPreFilter(uint32_t *aPreFilter) override; \
  NS_IMETHOD GetMatchRoles(nsTArray<uint32_t >& _retval) override; \
  NS_IMETHOD Match(nsIAccessible *aAccessible, uint16_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETRAVERSALRULE \
  nsresult GetPreFilter(uint32_t *aPreFilter); \
  nsresult GetMatchRoles(nsTArray<uint32_t >& _retval); \
  nsresult Match(nsIAccessible *aAccessible, uint16_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETRAVERSALRULE(_to) \
  NS_IMETHOD GetPreFilter(uint32_t *aPreFilter) override { return _to GetPreFilter(aPreFilter); } \
  NS_IMETHOD GetMatchRoles(nsTArray<uint32_t >& _retval) override { return _to GetMatchRoles(_retval); } \
  NS_IMETHOD Match(nsIAccessible *aAccessible, uint16_t *_retval) override { return _to Match(aAccessible, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETRAVERSALRULE(_to) \
  NS_IMETHOD GetPreFilter(uint32_t *aPreFilter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreFilter(aPreFilter); } \
  NS_IMETHOD GetMatchRoles(nsTArray<uint32_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchRoles(_retval); } \
  NS_IMETHOD Match(nsIAccessible *aAccessible, uint16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Match(aAccessible, _retval); } 


#endif /* __gen_nsIAccessiblePivot_h__ */
