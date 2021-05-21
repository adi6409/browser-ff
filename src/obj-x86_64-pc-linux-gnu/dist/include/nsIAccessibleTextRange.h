/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextRange.idl
 */

#ifndef __gen_nsIAccessibleTextRange_h__
#define __gen_nsIAccessibleTextRange_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAccessible; /* forward declaration */

class nsIAccessibleText; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    nsIAccessibleTextRange */
#define NS_IACCESSIBLETEXTRANGE_IID_STR "c4515623-55f9-4543-a3d5-c1e9afa588f4"

#define NS_IACCESSIBLETEXTRANGE_IID \
  {0xc4515623, 0x55f9, 0x4543, \
    { 0xa3, 0xd5, 0xc1, 0xe9, 0xaf, 0xa5, 0x88, 0xf4 }}

class NS_NO_VTABLE nsIAccessibleTextRange : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETEXTRANGE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTextRange;

  /* readonly attribute nsIAccessibleText startContainer; */
  NS_IMETHOD GetStartContainer(nsIAccessibleText **aStartContainer) = 0;

  /* readonly attribute long startOffset; */
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) = 0;

  /* readonly attribute nsIAccessibleText endContainer; */
  NS_IMETHOD GetEndContainer(nsIAccessibleText **aEndContainer) = 0;

  /* readonly attribute long endOffset; */
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) = 0;

  /* readonly attribute nsIAccessible container; */
  NS_IMETHOD GetContainer(nsIAccessible **aContainer) = 0;

  /* readonly attribute nsIArray embeddedChildren; */
  NS_IMETHOD GetEmbeddedChildren(nsIArray **aEmbeddedChildren) = 0;

  /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
  NS_IMETHOD Compare(nsIAccessibleTextRange *aOtherRange, bool *_retval) = 0;

  enum {
    EndPoint_Start = 1U,
    EndPoint_End = 2U
  };

  /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
  NS_IMETHOD CompareEndPoints(uint32_t aEndPoint, nsIAccessibleTextRange *aOtherRange, uint32_t aOtherRangeEndPoint, int32_t *_retval) = 0;

  /* readonly attribute AString text; */
  NS_IMETHOD GetText(nsAString& aText) = 0;

  /* readonly attribute nsIArray bounds; */
  NS_IMETHOD GetBounds(nsIArray **aBounds) = 0;

  enum {
    FormatUnit = 0U,
    WordUnit = 1U,
    LineUnit = 2U,
    ParagraphUnit = 3U,
    PageUnit = 4U,
    DocumentUnit = 5U
  };

  /* void move (in unsigned long aUnit, in long aCount); */
  NS_IMETHOD Move(uint32_t aUnit, int32_t aCount) = 0;

  /* void moveStart (in unsigned long aUnit, in long aCount); */
  NS_IMETHOD MoveStart(uint32_t aUnit, int32_t aCount) = 0;

  /* void moveEnd (in unsigned long aUnit, in long aCount); */
  NS_IMETHOD MoveEnd(uint32_t aUnit, int32_t aCount) = 0;

  /* void normalize (in unsigned long aUnit); */
  NS_IMETHOD Normalize(uint32_t aUnit) = 0;

  /* boolean crop (in nsIAccessible aContainer); */
  NS_IMETHOD Crop(nsIAccessible *aContainer, bool *_retval) = 0;

  /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
  NS_IMETHOD FindText(const nsAString& aText, bool aIsBackward, bool aIsIgnoreCase, nsIAccessibleTextRange **_retval) = 0;

  enum {
    AnimationStyleAttr = 0U,
    AnnotationObjectsAttr = 1U,
    AnnotationTypesAttr = 2U,
    BackgroundColorAttr = 3U,
    BulletStyleAttr = 4U,
    CapStyleAttr = 5U,
    CaretBidiModeAttr = 6U,
    CaretPositionAttr = 7U,
    CultureAttr = 8U,
    FontNameAttr = 9U,
    FontSizeAttr = 10U,
    FontWeightAttr = 11U,
    ForegroundColorAttr = 12U,
    HorizontalTextAlignmentAttr = 13U,
    IndentationFirstLineAttr = 14U,
    IndentationLeadingAttr = 15U,
    IndentationTrailingAttr = 16U,
    IsActiveAttr = 17U,
    IsHiddenAttr = 18U,
    IsItalicAttr = 19U,
    IsReadOnlyAttr = 20U,
    IsSubscriptAttr = 21U,
    IsSuperscriptAttr = 22U,
    LinkAttr = 23U,
    MarginBottomAttr = 24U,
    MarginLeadingAttr = 25U,
    MarginTopAttr = 26U,
    MarginTrailingAttr = 27U,
    OutlineStylesAttr = 28U,
    OverlineColorAttr = 29U,
    OverlineStyleAttr = 30U,
    SelectionActiveEndAttr = 31U,
    StrikethroughColorAttr = 32U,
    StrikethroughStyleAttr = 33U,
    StyleIdAttr = 34U,
    StyleNameAttr = 35U,
    TabsAttr = 36U,
    TextFlowDirectionsAttr = 37U,
    UnderlineColorAttr = 38U,
    UnderlineStyleAttr = 39U
  };

  /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
  NS_IMETHOD FindAttr(uint32_t aAttr, nsIVariant *aValue, bool aIsBackward, nsIAccessibleTextRange **_retval) = 0;

  /* void addToSelection (); */
  NS_IMETHOD AddToSelection(void) = 0;

  /* void removeFromSelection (); */
  NS_IMETHOD RemoveFromSelection(void) = 0;

  /* void select (); */
  NS_IMETHOD Select(void) = 0;

  enum {
    AlignToTop = 0U,
    AlignToBottom = 1U
  };

  /* void scrollIntoView (in unsigned long aHow); */
  NS_IMETHOD ScrollIntoView(uint32_t aHow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTextRange, NS_IACCESSIBLETEXTRANGE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETEXTRANGE \
  NS_IMETHOD GetStartContainer(nsIAccessibleText **aStartContainer) override; \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override; \
  NS_IMETHOD GetEndContainer(nsIAccessibleText **aEndContainer) override; \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override; \
  NS_IMETHOD GetContainer(nsIAccessible **aContainer) override; \
  NS_IMETHOD GetEmbeddedChildren(nsIArray **aEmbeddedChildren) override; \
  NS_IMETHOD Compare(nsIAccessibleTextRange *aOtherRange, bool *_retval) override; \
  NS_IMETHOD CompareEndPoints(uint32_t aEndPoint, nsIAccessibleTextRange *aOtherRange, uint32_t aOtherRangeEndPoint, int32_t *_retval) override; \
  NS_IMETHOD GetText(nsAString& aText) override; \
  NS_IMETHOD GetBounds(nsIArray **aBounds) override; \
  NS_IMETHOD Move(uint32_t aUnit, int32_t aCount) override; \
  NS_IMETHOD MoveStart(uint32_t aUnit, int32_t aCount) override; \
  NS_IMETHOD MoveEnd(uint32_t aUnit, int32_t aCount) override; \
  NS_IMETHOD Normalize(uint32_t aUnit) override; \
  NS_IMETHOD Crop(nsIAccessible *aContainer, bool *_retval) override; \
  NS_IMETHOD FindText(const nsAString& aText, bool aIsBackward, bool aIsIgnoreCase, nsIAccessibleTextRange **_retval) override; \
  NS_IMETHOD FindAttr(uint32_t aAttr, nsIVariant *aValue, bool aIsBackward, nsIAccessibleTextRange **_retval) override; \
  NS_IMETHOD AddToSelection(void) override; \
  NS_IMETHOD RemoveFromSelection(void) override; \
  NS_IMETHOD Select(void) override; \
  NS_IMETHOD ScrollIntoView(uint32_t aHow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETEXTRANGE \
  nsresult GetStartContainer(nsIAccessibleText **aStartContainer); \
  nsresult GetStartOffset(int32_t *aStartOffset); \
  nsresult GetEndContainer(nsIAccessibleText **aEndContainer); \
  nsresult GetEndOffset(int32_t *aEndOffset); \
  nsresult GetContainer(nsIAccessible **aContainer); \
  nsresult GetEmbeddedChildren(nsIArray **aEmbeddedChildren); \
  nsresult Compare(nsIAccessibleTextRange *aOtherRange, bool *_retval); \
  nsresult CompareEndPoints(uint32_t aEndPoint, nsIAccessibleTextRange *aOtherRange, uint32_t aOtherRangeEndPoint, int32_t *_retval); \
  nsresult GetText(nsAString& aText); \
  nsresult GetBounds(nsIArray **aBounds); \
  nsresult Move(uint32_t aUnit, int32_t aCount); \
  nsresult MoveStart(uint32_t aUnit, int32_t aCount); \
  nsresult MoveEnd(uint32_t aUnit, int32_t aCount); \
  nsresult Normalize(uint32_t aUnit); \
  nsresult Crop(nsIAccessible *aContainer, bool *_retval); \
  nsresult FindText(const nsAString& aText, bool aIsBackward, bool aIsIgnoreCase, nsIAccessibleTextRange **_retval); \
  nsresult FindAttr(uint32_t aAttr, nsIVariant *aValue, bool aIsBackward, nsIAccessibleTextRange **_retval); \
  nsresult AddToSelection(void); \
  nsresult RemoveFromSelection(void); \
  nsresult Select(void); \
  nsresult ScrollIntoView(uint32_t aHow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETEXTRANGE(_to) \
  NS_IMETHOD GetStartContainer(nsIAccessibleText **aStartContainer) override { return _to GetStartContainer(aStartContainer); } \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override { return _to GetStartOffset(aStartOffset); } \
  NS_IMETHOD GetEndContainer(nsIAccessibleText **aEndContainer) override { return _to GetEndContainer(aEndContainer); } \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override { return _to GetEndOffset(aEndOffset); } \
  NS_IMETHOD GetContainer(nsIAccessible **aContainer) override { return _to GetContainer(aContainer); } \
  NS_IMETHOD GetEmbeddedChildren(nsIArray **aEmbeddedChildren) override { return _to GetEmbeddedChildren(aEmbeddedChildren); } \
  NS_IMETHOD Compare(nsIAccessibleTextRange *aOtherRange, bool *_retval) override { return _to Compare(aOtherRange, _retval); } \
  NS_IMETHOD CompareEndPoints(uint32_t aEndPoint, nsIAccessibleTextRange *aOtherRange, uint32_t aOtherRangeEndPoint, int32_t *_retval) override { return _to CompareEndPoints(aEndPoint, aOtherRange, aOtherRangeEndPoint, _retval); } \
  NS_IMETHOD GetText(nsAString& aText) override { return _to GetText(aText); } \
  NS_IMETHOD GetBounds(nsIArray **aBounds) override { return _to GetBounds(aBounds); } \
  NS_IMETHOD Move(uint32_t aUnit, int32_t aCount) override { return _to Move(aUnit, aCount); } \
  NS_IMETHOD MoveStart(uint32_t aUnit, int32_t aCount) override { return _to MoveStart(aUnit, aCount); } \
  NS_IMETHOD MoveEnd(uint32_t aUnit, int32_t aCount) override { return _to MoveEnd(aUnit, aCount); } \
  NS_IMETHOD Normalize(uint32_t aUnit) override { return _to Normalize(aUnit); } \
  NS_IMETHOD Crop(nsIAccessible *aContainer, bool *_retval) override { return _to Crop(aContainer, _retval); } \
  NS_IMETHOD FindText(const nsAString& aText, bool aIsBackward, bool aIsIgnoreCase, nsIAccessibleTextRange **_retval) override { return _to FindText(aText, aIsBackward, aIsIgnoreCase, _retval); } \
  NS_IMETHOD FindAttr(uint32_t aAttr, nsIVariant *aValue, bool aIsBackward, nsIAccessibleTextRange **_retval) override { return _to FindAttr(aAttr, aValue, aIsBackward, _retval); } \
  NS_IMETHOD AddToSelection(void) override { return _to AddToSelection(); } \
  NS_IMETHOD RemoveFromSelection(void) override { return _to RemoveFromSelection(); } \
  NS_IMETHOD Select(void) override { return _to Select(); } \
  NS_IMETHOD ScrollIntoView(uint32_t aHow) override { return _to ScrollIntoView(aHow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETEXTRANGE(_to) \
  NS_IMETHOD GetStartContainer(nsIAccessibleText **aStartContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartContainer(aStartContainer); } \
  NS_IMETHOD GetStartOffset(int32_t *aStartOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartOffset(aStartOffset); } \
  NS_IMETHOD GetEndContainer(nsIAccessibleText **aEndContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndContainer(aEndContainer); } \
  NS_IMETHOD GetEndOffset(int32_t *aEndOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndOffset(aEndOffset); } \
  NS_IMETHOD GetContainer(nsIAccessible **aContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContainer(aContainer); } \
  NS_IMETHOD GetEmbeddedChildren(nsIArray **aEmbeddedChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEmbeddedChildren(aEmbeddedChildren); } \
  NS_IMETHOD Compare(nsIAccessibleTextRange *aOtherRange, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Compare(aOtherRange, _retval); } \
  NS_IMETHOD CompareEndPoints(uint32_t aEndPoint, nsIAccessibleTextRange *aOtherRange, uint32_t aOtherRangeEndPoint, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompareEndPoints(aEndPoint, aOtherRange, aOtherRangeEndPoint, _retval); } \
  NS_IMETHOD GetText(nsAString& aText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetText(aText); } \
  NS_IMETHOD GetBounds(nsIArray **aBounds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBounds(aBounds); } \
  NS_IMETHOD Move(uint32_t aUnit, int32_t aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Move(aUnit, aCount); } \
  NS_IMETHOD MoveStart(uint32_t aUnit, int32_t aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveStart(aUnit, aCount); } \
  NS_IMETHOD MoveEnd(uint32_t aUnit, int32_t aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveEnd(aUnit, aCount); } \
  NS_IMETHOD Normalize(uint32_t aUnit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Normalize(aUnit); } \
  NS_IMETHOD Crop(nsIAccessible *aContainer, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Crop(aContainer, _retval); } \
  NS_IMETHOD FindText(const nsAString& aText, bool aIsBackward, bool aIsIgnoreCase, nsIAccessibleTextRange **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindText(aText, aIsBackward, aIsIgnoreCase, _retval); } \
  NS_IMETHOD FindAttr(uint32_t aAttr, nsIVariant *aValue, bool aIsBackward, nsIAccessibleTextRange **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindAttr(aAttr, aValue, aIsBackward, _retval); } \
  NS_IMETHOD AddToSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddToSelection(); } \
  NS_IMETHOD RemoveFromSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveFromSelection(); } \
  NS_IMETHOD Select(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Select(); } \
  NS_IMETHOD ScrollIntoView(uint32_t aHow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollIntoView(aHow); } 


#endif /* __gen_nsIAccessibleTextRange_h__ */
