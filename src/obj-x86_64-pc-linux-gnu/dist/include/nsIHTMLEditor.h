/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLEditor.idl
 */

#ifndef __gen_nsIHTMLEditor_h__
#define __gen_nsIHTMLEditor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIContent; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

namespace mozilla {
namespace dom {
class Selection; /* webidl Selection */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIHTMLEditor */
#define NS_IHTMLEDITOR_IID_STR "87ee993e-985f-4a43-a974-0d9512da2fb0"

#define NS_IHTMLEDITOR_IID \
  {0x87ee993e, 0x985f, 0x4a43, \
    { 0xa9, 0x74, 0x0d, 0x95, 0x12, 0xda, 0x2f, 0xb0 }}

class nsIHTMLEditor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTMLEDITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHTMLEditor;

   typedef short EAlignment;
  enum {
    eLeft = 0,
    eCenter = 1,
    eRight = 2,
    eJustify = 3
  };

  /* [can_run_script] void setInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue) = 0;

  /* [can_run_script] void getInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll) = 0;

  /* [can_run_script] AString getInlinePropertyWithAttrValue (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlinePropertyWithAttrValue(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll, nsAString& _retval) = 0;

  /* [can_run_script] void removeInlineProperty (in AString aProperty, in AString aAttribute); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveInlineProperty(const nsAString& aProperty, const nsAString& aAttribute) = 0;

  /* boolean nodeIsBlock (in Node node); */
  NS_IMETHOD NodeIsBlock(nsINode *node, bool *_retval) = 0;

  /* [can_run_script] void insertHTML (in AString aInputString); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertHTML(const nsAString& aInputString) = 0;

  /* [can_run_script] void pasteNoFormatting (in long aSelectionType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteNoFormatting(int32_t aSelectionType) = 0;

  /* [can_run_script] void rebuildDocumentFromSource (in AString aSourceString); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RebuildDocumentFromSource(const nsAString& aSourceString) = 0;

  /* [can_run_script] void insertElementAtSelection (in Element aElement, in boolean aDeleteSelection); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertElementAtSelection(mozilla::dom::Element *aElement, bool aDeleteSelection) = 0;

  /* void updateBaseURL (); */
  NS_IMETHOD UpdateBaseURL(void) = 0;

  /* [can_run_script] void selectElement (in Element aElement); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectElement(mozilla::dom::Element *aElement) = 0;

  /* void setCaretAfterElement (in Element aElement); */
  NS_IMETHOD SetCaretAfterElement(mozilla::dom::Element *aElement) = 0;

  /* AString getParagraphState (out boolean aMixed); */
  NS_IMETHOD GetParagraphState(bool *aMixed, nsAString& _retval) = 0;

  /* [can_run_script] AString getFontFaceState (out boolean aMixed); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFontFaceState(bool *aMixed, nsAString& _retval) = 0;

  /* [can_run_script] AString getHighlightColorState (out boolean aMixed); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetHighlightColorState(bool *aMixed, nsAString& _retval) = 0;

  /* void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL); */
  NS_IMETHOD GetListState(bool *aMixed, bool *aOL, bool *aUL, bool *aDL) = 0;

  /* void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD); */
  NS_IMETHOD GetListItemState(bool *aMixed, bool *aLI, bool *aDT, bool *aDD) = 0;

  /* [can_run_script] void getAlignment (out boolean aMixed, out short aAlign); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlignment(bool *aMixed, int16_t *aAlign) = 0;

  /* [can_run_script] void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeOrChangeList(const nsAString& aListType, bool entireList, const nsAString& aBulletType) = 0;

  /* [can_run_script] void removeList (in AString aListType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveList(const nsAString& aListType) = 0;

  /* Element getElementOrParentByTagName (in AString aTagName, in Node aNode); */
  NS_IMETHOD GetElementOrParentByTagName(const nsAString& aTagName, nsINode *aNode, mozilla::dom::Element **_retval) = 0;

  /* nsISupports getSelectedElement (in AString aTagName); */
  NS_IMETHOD GetSelectedElement(const nsAString& aTagName, nsISupports **_retval) = 0;

  /* [can_run_script] Element createElementWithDefaults (in AString aTagName); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateElementWithDefaults(const nsAString& aTagName, mozilla::dom::Element **_retval) = 0;

  /* [can_run_script] void insertLinkAroundSelection (in Element aAnchorElement); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLinkAroundSelection(mozilla::dom::Element *aAnchorElement) = 0;

  /* [can_run_script] void setBackgroundColor (in AString aColor); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetBackgroundColor(const nsAString& aColor) = 0;

  /* [setter_can_run_script] attribute boolean isCSSEnabled; */
  NS_IMETHOD GetIsCSSEnabled(bool *aIsCSSEnabled) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsCSSEnabled(bool aIsCSSEnabled) = 0;

  /* [can_run_script] void checkSelectionStateForAnonymousButtons (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CheckSelectionStateForAnonymousButtons(void) = 0;

  /* boolean isAnonymousElement (in Element aElement); */
  NS_IMETHOD IsAnonymousElement(mozilla::dom::Element *aElement, bool *_retval) = 0;

  /* attribute boolean returnInParagraphCreatesNewParagraph; */
  NS_IMETHOD GetReturnInParagraphCreatesNewParagraph(bool *aReturnInParagraphCreatesNewParagraph) = 0;
  NS_IMETHOD SetReturnInParagraphCreatesNewParagraph(bool aReturnInParagraphCreatesNewParagraph) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHTMLEditor, NS_IHTMLEDITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTMLEDITOR \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlinePropertyWithAttrValue(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll, nsAString& _retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveInlineProperty(const nsAString& aProperty, const nsAString& aAttribute) override; \
  NS_IMETHOD NodeIsBlock(nsINode *node, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertHTML(const nsAString& aInputString) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteNoFormatting(int32_t aSelectionType) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RebuildDocumentFromSource(const nsAString& aSourceString) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertElementAtSelection(mozilla::dom::Element *aElement, bool aDeleteSelection) override; \
  NS_IMETHOD UpdateBaseURL(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectElement(mozilla::dom::Element *aElement) override; \
  NS_IMETHOD SetCaretAfterElement(mozilla::dom::Element *aElement) override; \
  NS_IMETHOD GetParagraphState(bool *aMixed, nsAString& _retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFontFaceState(bool *aMixed, nsAString& _retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetHighlightColorState(bool *aMixed, nsAString& _retval) override; \
  NS_IMETHOD GetListState(bool *aMixed, bool *aOL, bool *aUL, bool *aDL) override; \
  NS_IMETHOD GetListItemState(bool *aMixed, bool *aLI, bool *aDT, bool *aDD) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlignment(bool *aMixed, int16_t *aAlign) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeOrChangeList(const nsAString& aListType, bool entireList, const nsAString& aBulletType) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveList(const nsAString& aListType) override; \
  NS_IMETHOD GetElementOrParentByTagName(const nsAString& aTagName, nsINode *aNode, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD GetSelectedElement(const nsAString& aTagName, nsISupports **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateElementWithDefaults(const nsAString& aTagName, mozilla::dom::Element **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLinkAroundSelection(mozilla::dom::Element *aAnchorElement) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetBackgroundColor(const nsAString& aColor) override; \
  NS_IMETHOD GetIsCSSEnabled(bool *aIsCSSEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsCSSEnabled(bool aIsCSSEnabled) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CheckSelectionStateForAnonymousButtons(void) override; \
  NS_IMETHOD IsAnonymousElement(mozilla::dom::Element *aElement, bool *_retval) override; \
  NS_IMETHOD GetReturnInParagraphCreatesNewParagraph(bool *aReturnInParagraphCreatesNewParagraph) override; \
  NS_IMETHOD SetReturnInParagraphCreatesNewParagraph(bool aReturnInParagraphCreatesNewParagraph) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTMLEDITOR \
  MOZ_CAN_RUN_SCRIPT nsresult SetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue); \
  MOZ_CAN_RUN_SCRIPT nsresult GetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll); \
  MOZ_CAN_RUN_SCRIPT nsresult GetInlinePropertyWithAttrValue(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll, nsAString& _retval); \
  MOZ_CAN_RUN_SCRIPT nsresult RemoveInlineProperty(const nsAString& aProperty, const nsAString& aAttribute); \
  nsresult NodeIsBlock(nsINode *node, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertHTML(const nsAString& aInputString); \
  MOZ_CAN_RUN_SCRIPT nsresult PasteNoFormatting(int32_t aSelectionType); \
  MOZ_CAN_RUN_SCRIPT nsresult RebuildDocumentFromSource(const nsAString& aSourceString); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertElementAtSelection(mozilla::dom::Element *aElement, bool aDeleteSelection); \
  nsresult UpdateBaseURL(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectElement(mozilla::dom::Element *aElement); \
  nsresult SetCaretAfterElement(mozilla::dom::Element *aElement); \
  nsresult GetParagraphState(bool *aMixed, nsAString& _retval); \
  MOZ_CAN_RUN_SCRIPT nsresult GetFontFaceState(bool *aMixed, nsAString& _retval); \
  MOZ_CAN_RUN_SCRIPT nsresult GetHighlightColorState(bool *aMixed, nsAString& _retval); \
  nsresult GetListState(bool *aMixed, bool *aOL, bool *aUL, bool *aDL); \
  nsresult GetListItemState(bool *aMixed, bool *aLI, bool *aDT, bool *aDD); \
  MOZ_CAN_RUN_SCRIPT nsresult GetAlignment(bool *aMixed, int16_t *aAlign); \
  MOZ_CAN_RUN_SCRIPT nsresult MakeOrChangeList(const nsAString& aListType, bool entireList, const nsAString& aBulletType); \
  MOZ_CAN_RUN_SCRIPT nsresult RemoveList(const nsAString& aListType); \
  nsresult GetElementOrParentByTagName(const nsAString& aTagName, nsINode *aNode, mozilla::dom::Element **_retval); \
  nsresult GetSelectedElement(const nsAString& aTagName, nsISupports **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult CreateElementWithDefaults(const nsAString& aTagName, mozilla::dom::Element **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertLinkAroundSelection(mozilla::dom::Element *aAnchorElement); \
  MOZ_CAN_RUN_SCRIPT nsresult SetBackgroundColor(const nsAString& aColor); \
  nsresult GetIsCSSEnabled(bool *aIsCSSEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult SetIsCSSEnabled(bool aIsCSSEnabled); \
  MOZ_CAN_RUN_SCRIPT nsresult CheckSelectionStateForAnonymousButtons(void); \
  nsresult IsAnonymousElement(mozilla::dom::Element *aElement, bool *_retval); \
  nsresult GetReturnInParagraphCreatesNewParagraph(bool *aReturnInParagraphCreatesNewParagraph); \
  nsresult SetReturnInParagraphCreatesNewParagraph(bool aReturnInParagraphCreatesNewParagraph); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTMLEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue) override { return _to SetInlineProperty(aProperty, aAttribute, aValue); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll) override { return _to GetInlineProperty(aProperty, aAttribute, aValue, aFirst, aAny, aAll); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlinePropertyWithAttrValue(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll, nsAString& _retval) override { return _to GetInlinePropertyWithAttrValue(aProperty, aAttribute, aValue, aFirst, aAny, aAll, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveInlineProperty(const nsAString& aProperty, const nsAString& aAttribute) override { return _to RemoveInlineProperty(aProperty, aAttribute); } \
  NS_IMETHOD NodeIsBlock(nsINode *node, bool *_retval) override { return _to NodeIsBlock(node, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertHTML(const nsAString& aInputString) override { return _to InsertHTML(aInputString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteNoFormatting(int32_t aSelectionType) override { return _to PasteNoFormatting(aSelectionType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RebuildDocumentFromSource(const nsAString& aSourceString) override { return _to RebuildDocumentFromSource(aSourceString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertElementAtSelection(mozilla::dom::Element *aElement, bool aDeleteSelection) override { return _to InsertElementAtSelection(aElement, aDeleteSelection); } \
  NS_IMETHOD UpdateBaseURL(void) override { return _to UpdateBaseURL(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectElement(mozilla::dom::Element *aElement) override { return _to SelectElement(aElement); } \
  NS_IMETHOD SetCaretAfterElement(mozilla::dom::Element *aElement) override { return _to SetCaretAfterElement(aElement); } \
  NS_IMETHOD GetParagraphState(bool *aMixed, nsAString& _retval) override { return _to GetParagraphState(aMixed, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFontFaceState(bool *aMixed, nsAString& _retval) override { return _to GetFontFaceState(aMixed, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetHighlightColorState(bool *aMixed, nsAString& _retval) override { return _to GetHighlightColorState(aMixed, _retval); } \
  NS_IMETHOD GetListState(bool *aMixed, bool *aOL, bool *aUL, bool *aDL) override { return _to GetListState(aMixed, aOL, aUL, aDL); } \
  NS_IMETHOD GetListItemState(bool *aMixed, bool *aLI, bool *aDT, bool *aDD) override { return _to GetListItemState(aMixed, aLI, aDT, aDD); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlignment(bool *aMixed, int16_t *aAlign) override { return _to GetAlignment(aMixed, aAlign); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeOrChangeList(const nsAString& aListType, bool entireList, const nsAString& aBulletType) override { return _to MakeOrChangeList(aListType, entireList, aBulletType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveList(const nsAString& aListType) override { return _to RemoveList(aListType); } \
  NS_IMETHOD GetElementOrParentByTagName(const nsAString& aTagName, nsINode *aNode, mozilla::dom::Element **_retval) override { return _to GetElementOrParentByTagName(aTagName, aNode, _retval); } \
  NS_IMETHOD GetSelectedElement(const nsAString& aTagName, nsISupports **_retval) override { return _to GetSelectedElement(aTagName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateElementWithDefaults(const nsAString& aTagName, mozilla::dom::Element **_retval) override { return _to CreateElementWithDefaults(aTagName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLinkAroundSelection(mozilla::dom::Element *aAnchorElement) override { return _to InsertLinkAroundSelection(aAnchorElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetBackgroundColor(const nsAString& aColor) override { return _to SetBackgroundColor(aColor); } \
  NS_IMETHOD GetIsCSSEnabled(bool *aIsCSSEnabled) override { return _to GetIsCSSEnabled(aIsCSSEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsCSSEnabled(bool aIsCSSEnabled) override { return _to SetIsCSSEnabled(aIsCSSEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CheckSelectionStateForAnonymousButtons(void) override { return _to CheckSelectionStateForAnonymousButtons(); } \
  NS_IMETHOD IsAnonymousElement(mozilla::dom::Element *aElement, bool *_retval) override { return _to IsAnonymousElement(aElement, _retval); } \
  NS_IMETHOD GetReturnInParagraphCreatesNewParagraph(bool *aReturnInParagraphCreatesNewParagraph) override { return _to GetReturnInParagraphCreatesNewParagraph(aReturnInParagraphCreatesNewParagraph); } \
  NS_IMETHOD SetReturnInParagraphCreatesNewParagraph(bool aReturnInParagraphCreatesNewParagraph) override { return _to SetReturnInParagraphCreatesNewParagraph(aReturnInParagraphCreatesNewParagraph); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTMLEDITOR(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInlineProperty(aProperty, aAttribute, aValue); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlineProperty(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInlineProperty(aProperty, aAttribute, aValue, aFirst, aAny, aAll); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetInlinePropertyWithAttrValue(const nsAString& aProperty, const nsAString& aAttribute, const nsAString& aValue, bool *aFirst, bool *aAny, bool *aAll, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInlinePropertyWithAttrValue(aProperty, aAttribute, aValue, aFirst, aAny, aAll, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveInlineProperty(const nsAString& aProperty, const nsAString& aAttribute) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveInlineProperty(aProperty, aAttribute); } \
  NS_IMETHOD NodeIsBlock(nsINode *node, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodeIsBlock(node, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertHTML(const nsAString& aInputString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertHTML(aInputString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteNoFormatting(int32_t aSelectionType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PasteNoFormatting(aSelectionType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RebuildDocumentFromSource(const nsAString& aSourceString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RebuildDocumentFromSource(aSourceString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertElementAtSelection(mozilla::dom::Element *aElement, bool aDeleteSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertElementAtSelection(aElement, aDeleteSelection); } \
  NS_IMETHOD UpdateBaseURL(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateBaseURL(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectElement(mozilla::dom::Element *aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectElement(aElement); } \
  NS_IMETHOD SetCaretAfterElement(mozilla::dom::Element *aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaretAfterElement(aElement); } \
  NS_IMETHOD GetParagraphState(bool *aMixed, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParagraphState(aMixed, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetFontFaceState(bool *aMixed, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFontFaceState(aMixed, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetHighlightColorState(bool *aMixed, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHighlightColorState(aMixed, _retval); } \
  NS_IMETHOD GetListState(bool *aMixed, bool *aOL, bool *aUL, bool *aDL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListState(aMixed, aOL, aUL, aDL); } \
  NS_IMETHOD GetListItemState(bool *aMixed, bool *aLI, bool *aDT, bool *aDD) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListItemState(aMixed, aLI, aDT, aDD); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlignment(bool *aMixed, int16_t *aAlign) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlignment(aMixed, aAlign); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MakeOrChangeList(const nsAString& aListType, bool entireList, const nsAString& aBulletType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeOrChangeList(aListType, entireList, aBulletType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveList(const nsAString& aListType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveList(aListType); } \
  NS_IMETHOD GetElementOrParentByTagName(const nsAString& aTagName, nsINode *aNode, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetElementOrParentByTagName(aTagName, aNode, _retval); } \
  NS_IMETHOD GetSelectedElement(const nsAString& aTagName, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedElement(aTagName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CreateElementWithDefaults(const nsAString& aTagName, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateElementWithDefaults(aTagName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLinkAroundSelection(mozilla::dom::Element *aAnchorElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertLinkAroundSelection(aAnchorElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetBackgroundColor(const nsAString& aColor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBackgroundColor(aColor); } \
  NS_IMETHOD GetIsCSSEnabled(bool *aIsCSSEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsCSSEnabled(aIsCSSEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsCSSEnabled(bool aIsCSSEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsCSSEnabled(aIsCSSEnabled); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CheckSelectionStateForAnonymousButtons(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckSelectionStateForAnonymousButtons(); } \
  NS_IMETHOD IsAnonymousElement(mozilla::dom::Element *aElement, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAnonymousElement(aElement, _retval); } \
  NS_IMETHOD GetReturnInParagraphCreatesNewParagraph(bool *aReturnInParagraphCreatesNewParagraph) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReturnInParagraphCreatesNewParagraph(aReturnInParagraphCreatesNewParagraph); } \
  NS_IMETHOD SetReturnInParagraphCreatesNewParagraph(bool aReturnInParagraphCreatesNewParagraph) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReturnInParagraphCreatesNewParagraph(aReturnInParagraphCreatesNewParagraph); } 


#endif /* __gen_nsIHTMLEditor_h__ */
