/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditor.idl
 */

#ifndef __gen_nsIEditor_h__
#define __gen_nsIEditor_h__


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
class nsIURI; /* forward declaration */

class nsIContent; /* forward declaration */

class nsISelectionController; /* forward declaration */

class nsIDocumentStateListener; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsITransactionManager; /* forward declaration */

class nsITransaction; /* forward declaration */

class nsIEditorObserver; /* forward declaration */

class nsIEditActionListener; /* forward declaration */

class nsIInlineSpellChecker; /* forward declaration */

class nsITransferable; /* forward declaration */

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

namespace mozilla {
class EditorBase;
class HTMLEditor;
class TextEditor;
} // namespace mozilla

/* starting interface:    nsIEditor */
#define NS_IEDITOR_IID_STR "094be624-f0bf-400f-89e2-6a84baab9474"

#define NS_IEDITOR_IID \
  {0x094be624, 0xf0bf, 0x400f, \
    { 0x89, 0xe2, 0x6a, 0x84, 0xba, 0xab, 0x94, 0x74 }}

class nsIEditor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditor;

   typedef short EDirection;
  typedef short EStripWrappers;
  enum {
    eNone = 0,
    eNext = 1,
    ePrevious = 2,
    eNextWord = 3,
    ePreviousWord = 4,
    eToBeginningOfLine = 5,
    eToEndOfLine = 6,
    eStrip = 0,
    eNoStrip = 1,
    eEditorPlaintextMask = 1,
    eEditorSingleLineMask = 2,
    eEditorPasswordMask = 4,
    eEditorReadonlyMask = 8,
    eEditorFilterInputMask = 16,
    eEditorMailMask = 32,
    eEditorEnableWrapHackMask = 64,
    eEditorWidgetMask = 128,
    eEditorNoCSSMask = 256,
    eEditorAllowInteraction = 512,
    eEditorDontEchoPassword = 1024,
    eEditorRightToLeft = 2048,
    eEditorLeftToRight = 4096,
    eEditorSkipSpellCheck = 8192,
    eNewlinesPasteIntact = 0,
    eNewlinesPasteToFirst = 1,
    eNewlinesReplaceWithSpaces = 2,
    eNewlinesStrip = 3,
    eNewlinesReplaceWithCommas = 4,
    eNewlinesStripSurroundingWhitespace = 5
  };

  /* readonly attribute Selection selection; */
  NS_IMETHOD GetSelection(mozilla::dom::Selection **aSelection) = 0;

  /* [can_run_script] void setAttributeOrEquivalent (in Element element, in AString sourceAttrName, in AString sourceAttrValue, in boolean aSuppressTransaction); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, const nsAString& sourceAttrValue, bool aSuppressTransaction) = 0;

  /* [can_run_script] void removeAttributeOrEquivalent (in Element element, in AString sourceAttrName, in boolean aSuppressTransaction); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, bool aSuppressTransaction) = 0;

  /* [setter_can_run_script] attribute unsigned long flags; */
  NS_IMETHOD GetFlags(uint32_t *aFlags) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFlags(uint32_t aFlags) = 0;

  /* attribute AString contentsMIMEType; */
  NS_IMETHOD GetContentsMIMEType(nsAString& aContentsMIMEType) = 0;
  NS_IMETHOD SetContentsMIMEType(const nsAString& aContentsMIMEType) = 0;

  /* readonly attribute boolean isDocumentEditable; */
  NS_IMETHOD GetIsDocumentEditable(bool *aIsDocumentEditable) = 0;

  /* readonly attribute boolean isSelectionEditable; */
  NS_IMETHOD GetIsSelectionEditable(bool *aIsSelectionEditable) = 0;

  /* readonly attribute Document document; */
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) = 0;

  /* readonly attribute Element rootElement; */
  NS_IMETHOD GetRootElement(mozilla::dom::Element **aRootElement) = 0;

  /* readonly attribute nsISelectionController selectionController; */
  NS_IMETHOD GetSelectionController(nsISelectionController **aSelectionController) = 0;

  /* [can_run_script] void deleteSelection (in short action, in short stripWrappers); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteSelection(int16_t action, int16_t stripWrappers) = 0;

  /* readonly attribute boolean documentIsEmpty; */
  NS_IMETHOD GetDocumentIsEmpty(bool *aDocumentIsEmpty) = 0;

  /* readonly attribute boolean documentModified; */
  NS_IMETHOD GetDocumentModified(bool *aDocumentModified) = 0;

  /* [can_run_script] attribute ACString documentCharacterSet; */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocumentCharacterSet(nsACString& aDocumentCharacterSet) = 0;
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocumentCharacterSet(const nsACString& aDocumentCharacterSet) = 0;

  /* [can_run_script] void resetModificationCount (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetModificationCount(void) = 0;

  /* long getModificationCount (); */
  NS_IMETHOD GetModificationCount(int32_t *_retval) = 0;

  /* [can_run_script] void incrementModificationCount (in long aModCount); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD IncrementModificationCount(int32_t aModCount) = 0;

  /* readonly attribute nsITransactionManager transactionManager; */
  NS_IMETHOD GetTransactionManager(nsITransactionManager **aTransactionManager) = 0;

  /* [can_run_script] void doTransaction (in nsITransaction txn); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *txn) = 0;

  /* void enableUndo (in boolean enable); */
  NS_IMETHOD EnableUndo(bool enable) = 0;

  /* [can_run_script] void undo (in unsigned long count); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Undo(uint32_t count) = 0;

  /* void canUndo (out boolean isEnabled, out boolean canUndo); */
  NS_IMETHOD CanUndo(bool *isEnabled, bool *canUndo) = 0;

  /* [can_run_script] void redo (in unsigned long count); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Redo(uint32_t count) = 0;

  /* void canRedo (out boolean isEnabled, out boolean canRedo); */
  NS_IMETHOD CanRedo(bool *isEnabled, bool *canRedo) = 0;

  /* [can_run_script] void beginTransaction (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginTransaction(void) = 0;

  /* [can_run_script] void endTransaction (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndTransaction(void) = 0;

  /* void setShouldTxnSetSelection (in boolean should); */
  NS_IMETHOD SetShouldTxnSetSelection(bool should) = 0;

  /* nsIInlineSpellChecker getInlineSpellChecker (in boolean autoCreate); */
  NS_IMETHOD GetInlineSpellChecker(bool autoCreate, nsIInlineSpellChecker **_retval) = 0;

  /* void setSpellcheckUserOverride (in boolean enable); */
  NS_IMETHOD SetSpellcheckUserOverride(bool enable) = 0;

  /* [can_run_script] void cut (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cut(void) = 0;

  /* boolean canCut (); */
  NS_IMETHOD CanCut(bool *_retval) = 0;

  /* void copy (); */
  NS_IMETHOD Copy(void) = 0;

  /* boolean canCopy (); */
  NS_IMETHOD CanCopy(bool *_retval) = 0;

  /* [can_run_script] void paste (in long aClipboardType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Paste(int32_t aClipboardType) = 0;

  /* [can_run_script] void pasteTransferable (in nsITransferable aTransferable); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteTransferable(nsITransferable *aTransferable) = 0;

  /* boolean canPaste (in long aClipboardType); */
  NS_IMETHOD CanPaste(int32_t aClipboardType, bool *_retval) = 0;

  /* [can_run_script] void selectAll (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) = 0;

  /* void beginningOfDocument (); */
  NS_IMETHOD BeginningOfDocument(void) = 0;

  /* void endOfDocument (); */
  NS_IMETHOD EndOfDocument(void) = 0;

  /* [can_run_script] void setAttribute (in Element aElement, in AString attributestr, in AString attvalue); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttribute(mozilla::dom::Element *aElement, const nsAString& attributestr, const nsAString& attvalue) = 0;

  /* [can_run_script] void removeAttribute (in Element aElement, in AString aAttribute); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttribute(mozilla::dom::Element *aElement, const nsAString& aAttribute) = 0;

  /* [can_run_script] void cloneAttributes (in Element aDestElement, in Element aSourceElement); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CloneAttributes(mozilla::dom::Element *aDestElement, mozilla::dom::Element *aSourceElement) = 0;

  /* [can_run_script] void insertNode (in Node node, in Node parent, in long aPosition); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertNode(nsINode *node, nsINode *parent, int32_t aPosition) = 0;

  /* [can_run_script] void deleteNode (in Node child); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteNode(nsINode *child) = 0;

  /* AString outputToString (in AString formatType, in unsigned long flags); */
  NS_IMETHOD OutputToString(const nsAString& formatType, uint32_t flags, nsAString& _retval) = 0;

  /* void addEditorObserver (in nsIEditorObserver observer); */
  NS_IMETHOD AddEditorObserver(nsIEditorObserver *observer) = 0;

  /* void addEditActionListener (in nsIEditActionListener listener); */
  NS_IMETHOD AddEditActionListener(nsIEditActionListener *listener) = 0;

  /* void removeEditActionListener (in nsIEditActionListener listener); */
  NS_IMETHOD RemoveEditActionListener(nsIEditActionListener *listener) = 0;

  /* void addDocumentStateListener (in nsIDocumentStateListener listener); */
  NS_IMETHOD AddDocumentStateListener(nsIDocumentStateListener *listener) = 0;

  /* void removeDocumentStateListener (in nsIDocumentStateListener listener); */
  NS_IMETHOD RemoveDocumentStateListener(nsIDocumentStateListener *listener) = 0;

  /* void forceCompositionEnd (); */
  NS_IMETHOD ForceCompositionEnd(void) = 0;

  /* readonly attribute boolean composing; */
  NS_IMETHOD GetComposing(bool *aComposing) = 0;

  /* [can_run_script,optional_argc] void unmask ([optional] in unsigned long aStart, [optional] in long long aEnd, [optional] in unsigned long aTimeout); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Unmask(uint32_t aStart, int64_t aEnd, uint32_t aTimeout, uint8_t _argc) = 0;

  /* [can_run_script] void mask (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Mask(void) = 0;

  /* readonly attribute unsigned long unmaskedStart; */
  NS_IMETHOD GetUnmaskedStart(uint32_t *aUnmaskedStart) = 0;

  /* readonly attribute unsigned long unmaskedEnd; */
  NS_IMETHOD GetUnmaskedEnd(uint32_t *aUnmaskedEnd) = 0;

  /* readonly attribute boolean autoMaskingEnabled; */
  NS_IMETHOD GetAutoMaskingEnabled(bool *aAutoMaskingEnabled) = 0;

  /* readonly attribute AString passwordMask; */
  NS_IMETHOD GetPasswordMask(nsAString& aPasswordMask) = 0;

  /* readonly attribute long textLength; */
  NS_IMETHOD GetTextLength(int32_t *aTextLength) = 0;

  /* attribute long wrapWidth; */
  NS_IMETHOD GetWrapWidth(int32_t *aWrapWidth) = 0;
  NS_IMETHOD SetWrapWidth(int32_t aWrapWidth) = 0;

  /* attribute long newlineHandling; */
  NS_IMETHOD GetNewlineHandling(int32_t *aNewlineHandling) = 0;
  NS_IMETHOD SetNewlineHandling(int32_t aNewlineHandling) = 0;

  /* [can_run_script] void insertText (in AString aStringToInsert); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertText(const nsAString& aStringToInsert) = 0;

  /* [can_run_script] void insertLineBreak (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLineBreak(void) = 0;

   /**
   * AsEditorBase() returns a pointer to EditorBase class.
   *
   * In order to avoid circular dependency issues, this method is defined
   * in mozilla/EditorBase.h.  Consumers need to #include that header.
   */
  inline mozilla::EditorBase* AsEditorBase();
  inline const mozilla::EditorBase* AsEditorBase() const;
  /**
   * AsTextEditor() returns a pointer to TextEditor class.
   *
   * In order to avoid circular dependency issues, this method is defined
   * in mozilla/TextEditor.h.  Consumers need to #include that header.
   */
  inline mozilla::TextEditor* AsTextEditor();
  inline const mozilla::TextEditor* AsTextEditor() const;
  /**
   * AsHTMLEditor() returns a pointer to HTMLEditor class.
   *
   * In order to avoid circular dependency issues, this method is defined
   * in mozilla/HTMLEditor.h.  Consumers need to #include that header.
   */
  inline mozilla::HTMLEditor* AsHTMLEditor();
  inline const mozilla::HTMLEditor* AsHTMLEditor() const;
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditor, NS_IEDITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITOR \
  NS_IMETHOD GetSelection(mozilla::dom::Selection **aSelection) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, const nsAString& sourceAttrValue, bool aSuppressTransaction) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, bool aSuppressTransaction) override; \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFlags(uint32_t aFlags) override; \
  NS_IMETHOD GetContentsMIMEType(nsAString& aContentsMIMEType) override; \
  NS_IMETHOD SetContentsMIMEType(const nsAString& aContentsMIMEType) override; \
  NS_IMETHOD GetIsDocumentEditable(bool *aIsDocumentEditable) override; \
  NS_IMETHOD GetIsSelectionEditable(bool *aIsSelectionEditable) override; \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override; \
  NS_IMETHOD GetRootElement(mozilla::dom::Element **aRootElement) override; \
  NS_IMETHOD GetSelectionController(nsISelectionController **aSelectionController) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteSelection(int16_t action, int16_t stripWrappers) override; \
  NS_IMETHOD GetDocumentIsEmpty(bool *aDocumentIsEmpty) override; \
  NS_IMETHOD GetDocumentModified(bool *aDocumentModified) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocumentCharacterSet(nsACString& aDocumentCharacterSet) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocumentCharacterSet(const nsACString& aDocumentCharacterSet) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetModificationCount(void) override; \
  NS_IMETHOD GetModificationCount(int32_t *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD IncrementModificationCount(int32_t aModCount) override; \
  NS_IMETHOD GetTransactionManager(nsITransactionManager **aTransactionManager) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *txn) override; \
  NS_IMETHOD EnableUndo(bool enable) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Undo(uint32_t count) override; \
  NS_IMETHOD CanUndo(bool *isEnabled, bool *canUndo) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Redo(uint32_t count) override; \
  NS_IMETHOD CanRedo(bool *isEnabled, bool *canRedo) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginTransaction(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndTransaction(void) override; \
  NS_IMETHOD SetShouldTxnSetSelection(bool should) override; \
  NS_IMETHOD GetInlineSpellChecker(bool autoCreate, nsIInlineSpellChecker **_retval) override; \
  NS_IMETHOD SetSpellcheckUserOverride(bool enable) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cut(void) override; \
  NS_IMETHOD CanCut(bool *_retval) override; \
  NS_IMETHOD Copy(void) override; \
  NS_IMETHOD CanCopy(bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Paste(int32_t aClipboardType) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteTransferable(nsITransferable *aTransferable) override; \
  NS_IMETHOD CanPaste(int32_t aClipboardType, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) override; \
  NS_IMETHOD BeginningOfDocument(void) override; \
  NS_IMETHOD EndOfDocument(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttribute(mozilla::dom::Element *aElement, const nsAString& attributestr, const nsAString& attvalue) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttribute(mozilla::dom::Element *aElement, const nsAString& aAttribute) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CloneAttributes(mozilla::dom::Element *aDestElement, mozilla::dom::Element *aSourceElement) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertNode(nsINode *node, nsINode *parent, int32_t aPosition) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteNode(nsINode *child) override; \
  NS_IMETHOD OutputToString(const nsAString& formatType, uint32_t flags, nsAString& _retval) override; \
  NS_IMETHOD AddEditorObserver(nsIEditorObserver *observer) override; \
  NS_IMETHOD AddEditActionListener(nsIEditActionListener *listener) override; \
  NS_IMETHOD RemoveEditActionListener(nsIEditActionListener *listener) override; \
  NS_IMETHOD AddDocumentStateListener(nsIDocumentStateListener *listener) override; \
  NS_IMETHOD RemoveDocumentStateListener(nsIDocumentStateListener *listener) override; \
  NS_IMETHOD ForceCompositionEnd(void) override; \
  NS_IMETHOD GetComposing(bool *aComposing) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Unmask(uint32_t aStart, int64_t aEnd, uint32_t aTimeout, uint8_t _argc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Mask(void) override; \
  NS_IMETHOD GetUnmaskedStart(uint32_t *aUnmaskedStart) override; \
  NS_IMETHOD GetUnmaskedEnd(uint32_t *aUnmaskedEnd) override; \
  NS_IMETHOD GetAutoMaskingEnabled(bool *aAutoMaskingEnabled) override; \
  NS_IMETHOD GetPasswordMask(nsAString& aPasswordMask) override; \
  NS_IMETHOD GetTextLength(int32_t *aTextLength) override; \
  NS_IMETHOD GetWrapWidth(int32_t *aWrapWidth) override; \
  NS_IMETHOD SetWrapWidth(int32_t aWrapWidth) override; \
  NS_IMETHOD GetNewlineHandling(int32_t *aNewlineHandling) override; \
  NS_IMETHOD SetNewlineHandling(int32_t aNewlineHandling) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertText(const nsAString& aStringToInsert) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLineBreak(void) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITOR \
  nsresult GetSelection(mozilla::dom::Selection **aSelection); \
  MOZ_CAN_RUN_SCRIPT nsresult SetAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, const nsAString& sourceAttrValue, bool aSuppressTransaction); \
  MOZ_CAN_RUN_SCRIPT nsresult RemoveAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, bool aSuppressTransaction); \
  nsresult GetFlags(uint32_t *aFlags); \
  MOZ_CAN_RUN_SCRIPT nsresult SetFlags(uint32_t aFlags); \
  nsresult GetContentsMIMEType(nsAString& aContentsMIMEType); \
  nsresult SetContentsMIMEType(const nsAString& aContentsMIMEType); \
  nsresult GetIsDocumentEditable(bool *aIsDocumentEditable); \
  nsresult GetIsSelectionEditable(bool *aIsSelectionEditable); \
  nsresult GetDocument(mozilla::dom::Document **aDocument); \
  nsresult GetRootElement(mozilla::dom::Element **aRootElement); \
  nsresult GetSelectionController(nsISelectionController **aSelectionController); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteSelection(int16_t action, int16_t stripWrappers); \
  nsresult GetDocumentIsEmpty(bool *aDocumentIsEmpty); \
  nsresult GetDocumentModified(bool *aDocumentModified); \
  MOZ_CAN_RUN_SCRIPT nsresult GetDocumentCharacterSet(nsACString& aDocumentCharacterSet); \
  MOZ_CAN_RUN_SCRIPT nsresult SetDocumentCharacterSet(const nsACString& aDocumentCharacterSet); \
  MOZ_CAN_RUN_SCRIPT nsresult ResetModificationCount(void); \
  nsresult GetModificationCount(int32_t *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult IncrementModificationCount(int32_t aModCount); \
  nsresult GetTransactionManager(nsITransactionManager **aTransactionManager); \
  MOZ_CAN_RUN_SCRIPT nsresult DoTransaction(nsITransaction *txn); \
  nsresult EnableUndo(bool enable); \
  MOZ_CAN_RUN_SCRIPT nsresult Undo(uint32_t count); \
  nsresult CanUndo(bool *isEnabled, bool *canUndo); \
  MOZ_CAN_RUN_SCRIPT nsresult Redo(uint32_t count); \
  nsresult CanRedo(bool *isEnabled, bool *canRedo); \
  MOZ_CAN_RUN_SCRIPT nsresult BeginTransaction(void); \
  MOZ_CAN_RUN_SCRIPT nsresult EndTransaction(void); \
  nsresult SetShouldTxnSetSelection(bool should); \
  nsresult GetInlineSpellChecker(bool autoCreate, nsIInlineSpellChecker **_retval); \
  nsresult SetSpellcheckUserOverride(bool enable); \
  MOZ_CAN_RUN_SCRIPT nsresult Cut(void); \
  nsresult CanCut(bool *_retval); \
  nsresult Copy(void); \
  nsresult CanCopy(bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult Paste(int32_t aClipboardType); \
  MOZ_CAN_RUN_SCRIPT nsresult PasteTransferable(nsITransferable *aTransferable); \
  nsresult CanPaste(int32_t aClipboardType, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectAll(void); \
  nsresult BeginningOfDocument(void); \
  nsresult EndOfDocument(void); \
  MOZ_CAN_RUN_SCRIPT nsresult SetAttribute(mozilla::dom::Element *aElement, const nsAString& attributestr, const nsAString& attvalue); \
  MOZ_CAN_RUN_SCRIPT nsresult RemoveAttribute(mozilla::dom::Element *aElement, const nsAString& aAttribute); \
  MOZ_CAN_RUN_SCRIPT nsresult CloneAttributes(mozilla::dom::Element *aDestElement, mozilla::dom::Element *aSourceElement); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertNode(nsINode *node, nsINode *parent, int32_t aPosition); \
  MOZ_CAN_RUN_SCRIPT nsresult DeleteNode(nsINode *child); \
  nsresult OutputToString(const nsAString& formatType, uint32_t flags, nsAString& _retval); \
  nsresult AddEditorObserver(nsIEditorObserver *observer); \
  nsresult AddEditActionListener(nsIEditActionListener *listener); \
  nsresult RemoveEditActionListener(nsIEditActionListener *listener); \
  nsresult AddDocumentStateListener(nsIDocumentStateListener *listener); \
  nsresult RemoveDocumentStateListener(nsIDocumentStateListener *listener); \
  nsresult ForceCompositionEnd(void); \
  nsresult GetComposing(bool *aComposing); \
  MOZ_CAN_RUN_SCRIPT nsresult Unmask(uint32_t aStart, int64_t aEnd, uint32_t aTimeout, uint8_t _argc); \
  MOZ_CAN_RUN_SCRIPT nsresult Mask(void); \
  nsresult GetUnmaskedStart(uint32_t *aUnmaskedStart); \
  nsresult GetUnmaskedEnd(uint32_t *aUnmaskedEnd); \
  nsresult GetAutoMaskingEnabled(bool *aAutoMaskingEnabled); \
  nsresult GetPasswordMask(nsAString& aPasswordMask); \
  nsresult GetTextLength(int32_t *aTextLength); \
  nsresult GetWrapWidth(int32_t *aWrapWidth); \
  nsresult SetWrapWidth(int32_t aWrapWidth); \
  nsresult GetNewlineHandling(int32_t *aNewlineHandling); \
  nsresult SetNewlineHandling(int32_t aNewlineHandling); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertText(const nsAString& aStringToInsert); \
  MOZ_CAN_RUN_SCRIPT nsresult InsertLineBreak(void); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITOR(_to) \
  NS_IMETHOD GetSelection(mozilla::dom::Selection **aSelection) override { return _to GetSelection(aSelection); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, const nsAString& sourceAttrValue, bool aSuppressTransaction) override { return _to SetAttributeOrEquivalent(element, sourceAttrName, sourceAttrValue, aSuppressTransaction); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, bool aSuppressTransaction) override { return _to RemoveAttributeOrEquivalent(element, sourceAttrName, aSuppressTransaction); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return _to GetFlags(aFlags); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFlags(uint32_t aFlags) override { return _to SetFlags(aFlags); } \
  NS_IMETHOD GetContentsMIMEType(nsAString& aContentsMIMEType) override { return _to GetContentsMIMEType(aContentsMIMEType); } \
  NS_IMETHOD SetContentsMIMEType(const nsAString& aContentsMIMEType) override { return _to SetContentsMIMEType(aContentsMIMEType); } \
  NS_IMETHOD GetIsDocumentEditable(bool *aIsDocumentEditable) override { return _to GetIsDocumentEditable(aIsDocumentEditable); } \
  NS_IMETHOD GetIsSelectionEditable(bool *aIsSelectionEditable) override { return _to GetIsSelectionEditable(aIsSelectionEditable); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return _to GetDocument(aDocument); } \
  NS_IMETHOD GetRootElement(mozilla::dom::Element **aRootElement) override { return _to GetRootElement(aRootElement); } \
  NS_IMETHOD GetSelectionController(nsISelectionController **aSelectionController) override { return _to GetSelectionController(aSelectionController); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteSelection(int16_t action, int16_t stripWrappers) override { return _to DeleteSelection(action, stripWrappers); } \
  NS_IMETHOD GetDocumentIsEmpty(bool *aDocumentIsEmpty) override { return _to GetDocumentIsEmpty(aDocumentIsEmpty); } \
  NS_IMETHOD GetDocumentModified(bool *aDocumentModified) override { return _to GetDocumentModified(aDocumentModified); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocumentCharacterSet(nsACString& aDocumentCharacterSet) override { return _to GetDocumentCharacterSet(aDocumentCharacterSet); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocumentCharacterSet(const nsACString& aDocumentCharacterSet) override { return _to SetDocumentCharacterSet(aDocumentCharacterSet); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetModificationCount(void) override { return _to ResetModificationCount(); } \
  NS_IMETHOD GetModificationCount(int32_t *_retval) override { return _to GetModificationCount(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD IncrementModificationCount(int32_t aModCount) override { return _to IncrementModificationCount(aModCount); } \
  NS_IMETHOD GetTransactionManager(nsITransactionManager **aTransactionManager) override { return _to GetTransactionManager(aTransactionManager); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *txn) override { return _to DoTransaction(txn); } \
  NS_IMETHOD EnableUndo(bool enable) override { return _to EnableUndo(enable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Undo(uint32_t count) override { return _to Undo(count); } \
  NS_IMETHOD CanUndo(bool *isEnabled, bool *canUndo) override { return _to CanUndo(isEnabled, canUndo); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Redo(uint32_t count) override { return _to Redo(count); } \
  NS_IMETHOD CanRedo(bool *isEnabled, bool *canRedo) override { return _to CanRedo(isEnabled, canRedo); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginTransaction(void) override { return _to BeginTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndTransaction(void) override { return _to EndTransaction(); } \
  NS_IMETHOD SetShouldTxnSetSelection(bool should) override { return _to SetShouldTxnSetSelection(should); } \
  NS_IMETHOD GetInlineSpellChecker(bool autoCreate, nsIInlineSpellChecker **_retval) override { return _to GetInlineSpellChecker(autoCreate, _retval); } \
  NS_IMETHOD SetSpellcheckUserOverride(bool enable) override { return _to SetSpellcheckUserOverride(enable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cut(void) override { return _to Cut(); } \
  NS_IMETHOD CanCut(bool *_retval) override { return _to CanCut(_retval); } \
  NS_IMETHOD Copy(void) override { return _to Copy(); } \
  NS_IMETHOD CanCopy(bool *_retval) override { return _to CanCopy(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Paste(int32_t aClipboardType) override { return _to Paste(aClipboardType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteTransferable(nsITransferable *aTransferable) override { return _to PasteTransferable(aTransferable); } \
  NS_IMETHOD CanPaste(int32_t aClipboardType, bool *_retval) override { return _to CanPaste(aClipboardType, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) override { return _to SelectAll(); } \
  NS_IMETHOD BeginningOfDocument(void) override { return _to BeginningOfDocument(); } \
  NS_IMETHOD EndOfDocument(void) override { return _to EndOfDocument(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttribute(mozilla::dom::Element *aElement, const nsAString& attributestr, const nsAString& attvalue) override { return _to SetAttribute(aElement, attributestr, attvalue); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttribute(mozilla::dom::Element *aElement, const nsAString& aAttribute) override { return _to RemoveAttribute(aElement, aAttribute); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CloneAttributes(mozilla::dom::Element *aDestElement, mozilla::dom::Element *aSourceElement) override { return _to CloneAttributes(aDestElement, aSourceElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertNode(nsINode *node, nsINode *parent, int32_t aPosition) override { return _to InsertNode(node, parent, aPosition); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteNode(nsINode *child) override { return _to DeleteNode(child); } \
  NS_IMETHOD OutputToString(const nsAString& formatType, uint32_t flags, nsAString& _retval) override { return _to OutputToString(formatType, flags, _retval); } \
  NS_IMETHOD AddEditorObserver(nsIEditorObserver *observer) override { return _to AddEditorObserver(observer); } \
  NS_IMETHOD AddEditActionListener(nsIEditActionListener *listener) override { return _to AddEditActionListener(listener); } \
  NS_IMETHOD RemoveEditActionListener(nsIEditActionListener *listener) override { return _to RemoveEditActionListener(listener); } \
  NS_IMETHOD AddDocumentStateListener(nsIDocumentStateListener *listener) override { return _to AddDocumentStateListener(listener); } \
  NS_IMETHOD RemoveDocumentStateListener(nsIDocumentStateListener *listener) override { return _to RemoveDocumentStateListener(listener); } \
  NS_IMETHOD ForceCompositionEnd(void) override { return _to ForceCompositionEnd(); } \
  NS_IMETHOD GetComposing(bool *aComposing) override { return _to GetComposing(aComposing); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Unmask(uint32_t aStart, int64_t aEnd, uint32_t aTimeout, uint8_t _argc) override { return _to Unmask(aStart, aEnd, aTimeout, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Mask(void) override { return _to Mask(); } \
  NS_IMETHOD GetUnmaskedStart(uint32_t *aUnmaskedStart) override { return _to GetUnmaskedStart(aUnmaskedStart); } \
  NS_IMETHOD GetUnmaskedEnd(uint32_t *aUnmaskedEnd) override { return _to GetUnmaskedEnd(aUnmaskedEnd); } \
  NS_IMETHOD GetAutoMaskingEnabled(bool *aAutoMaskingEnabled) override { return _to GetAutoMaskingEnabled(aAutoMaskingEnabled); } \
  NS_IMETHOD GetPasswordMask(nsAString& aPasswordMask) override { return _to GetPasswordMask(aPasswordMask); } \
  NS_IMETHOD GetTextLength(int32_t *aTextLength) override { return _to GetTextLength(aTextLength); } \
  NS_IMETHOD GetWrapWidth(int32_t *aWrapWidth) override { return _to GetWrapWidth(aWrapWidth); } \
  NS_IMETHOD SetWrapWidth(int32_t aWrapWidth) override { return _to SetWrapWidth(aWrapWidth); } \
  NS_IMETHOD GetNewlineHandling(int32_t *aNewlineHandling) override { return _to GetNewlineHandling(aNewlineHandling); } \
  NS_IMETHOD SetNewlineHandling(int32_t aNewlineHandling) override { return _to SetNewlineHandling(aNewlineHandling); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertText(const nsAString& aStringToInsert) override { return _to InsertText(aStringToInsert); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLineBreak(void) override { return _to InsertLineBreak(); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITOR(_to) \
  NS_IMETHOD GetSelection(mozilla::dom::Selection **aSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelection(aSelection); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, const nsAString& sourceAttrValue, bool aSuppressTransaction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAttributeOrEquivalent(element, sourceAttrName, sourceAttrValue, aSuppressTransaction); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttributeOrEquivalent(mozilla::dom::Element *element, const nsAString& sourceAttrName, bool aSuppressTransaction) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAttributeOrEquivalent(element, sourceAttrName, aSuppressTransaction); } \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlags(aFlags); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFlags(uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFlags(aFlags); } \
  NS_IMETHOD GetContentsMIMEType(nsAString& aContentsMIMEType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentsMIMEType(aContentsMIMEType); } \
  NS_IMETHOD SetContentsMIMEType(const nsAString& aContentsMIMEType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetContentsMIMEType(aContentsMIMEType); } \
  NS_IMETHOD GetIsDocumentEditable(bool *aIsDocumentEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsDocumentEditable(aIsDocumentEditable); } \
  NS_IMETHOD GetIsSelectionEditable(bool *aIsSelectionEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSelectionEditable(aIsSelectionEditable); } \
  NS_IMETHOD GetDocument(mozilla::dom::Document **aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocument(aDocument); } \
  NS_IMETHOD GetRootElement(mozilla::dom::Element **aRootElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootElement(aRootElement); } \
  NS_IMETHOD GetSelectionController(nsISelectionController **aSelectionController) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionController(aSelectionController); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteSelection(int16_t action, int16_t stripWrappers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteSelection(action, stripWrappers); } \
  NS_IMETHOD GetDocumentIsEmpty(bool *aDocumentIsEmpty) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentIsEmpty(aDocumentIsEmpty); } \
  NS_IMETHOD GetDocumentModified(bool *aDocumentModified) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentModified(aDocumentModified); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetDocumentCharacterSet(nsACString& aDocumentCharacterSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentCharacterSet(aDocumentCharacterSet); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocumentCharacterSet(const nsACString& aDocumentCharacterSet) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocumentCharacterSet(aDocumentCharacterSet); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetModificationCount(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetModificationCount(); } \
  NS_IMETHOD GetModificationCount(int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModificationCount(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD IncrementModificationCount(int32_t aModCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IncrementModificationCount(aModCount); } \
  NS_IMETHOD GetTransactionManager(nsITransactionManager **aTransactionManager) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTransactionManager(aTransactionManager); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DoTransaction(nsITransaction *txn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoTransaction(txn); } \
  NS_IMETHOD EnableUndo(bool enable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableUndo(enable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Undo(uint32_t count) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Undo(count); } \
  NS_IMETHOD CanUndo(bool *isEnabled, bool *canUndo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanUndo(isEnabled, canUndo); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Redo(uint32_t count) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Redo(count); } \
  NS_IMETHOD CanRedo(bool *isEnabled, bool *canRedo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanRedo(isEnabled, canRedo); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD BeginTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginTransaction(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD EndTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndTransaction(); } \
  NS_IMETHOD SetShouldTxnSetSelection(bool should) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShouldTxnSetSelection(should); } \
  NS_IMETHOD GetInlineSpellChecker(bool autoCreate, nsIInlineSpellChecker **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInlineSpellChecker(autoCreate, _retval); } \
  NS_IMETHOD SetSpellcheckUserOverride(bool enable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSpellcheckUserOverride(enable); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Cut(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cut(); } \
  NS_IMETHOD CanCut(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanCut(_retval); } \
  NS_IMETHOD Copy(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Copy(); } \
  NS_IMETHOD CanCopy(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanCopy(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Paste(int32_t aClipboardType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Paste(aClipboardType); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PasteTransferable(nsITransferable *aTransferable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PasteTransferable(aTransferable); } \
  NS_IMETHOD CanPaste(int32_t aClipboardType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanPaste(aClipboardType, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAll(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAll(); } \
  NS_IMETHOD BeginningOfDocument(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginningOfDocument(); } \
  NS_IMETHOD EndOfDocument(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndOfDocument(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetAttribute(mozilla::dom::Element *aElement, const nsAString& attributestr, const nsAString& attvalue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAttribute(aElement, attributestr, attvalue); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveAttribute(mozilla::dom::Element *aElement, const nsAString& aAttribute) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveAttribute(aElement, aAttribute); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CloneAttributes(mozilla::dom::Element *aDestElement, mozilla::dom::Element *aSourceElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloneAttributes(aDestElement, aSourceElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertNode(nsINode *node, nsINode *parent, int32_t aPosition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertNode(node, parent, aPosition); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteNode(nsINode *child) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteNode(child); } \
  NS_IMETHOD OutputToString(const nsAString& formatType, uint32_t flags, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OutputToString(formatType, flags, _retval); } \
  NS_IMETHOD AddEditorObserver(nsIEditorObserver *observer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEditorObserver(observer); } \
  NS_IMETHOD AddEditActionListener(nsIEditActionListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEditActionListener(listener); } \
  NS_IMETHOD RemoveEditActionListener(nsIEditActionListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveEditActionListener(listener); } \
  NS_IMETHOD AddDocumentStateListener(nsIDocumentStateListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDocumentStateListener(listener); } \
  NS_IMETHOD RemoveDocumentStateListener(nsIDocumentStateListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDocumentStateListener(listener); } \
  NS_IMETHOD ForceCompositionEnd(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceCompositionEnd(); } \
  NS_IMETHOD GetComposing(bool *aComposing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetComposing(aComposing); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Unmask(uint32_t aStart, int64_t aEnd, uint32_t aTimeout, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unmask(aStart, aEnd, aTimeout, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Mask(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Mask(); } \
  NS_IMETHOD GetUnmaskedStart(uint32_t *aUnmaskedStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnmaskedStart(aUnmaskedStart); } \
  NS_IMETHOD GetUnmaskedEnd(uint32_t *aUnmaskedEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnmaskedEnd(aUnmaskedEnd); } \
  NS_IMETHOD GetAutoMaskingEnabled(bool *aAutoMaskingEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAutoMaskingEnabled(aAutoMaskingEnabled); } \
  NS_IMETHOD GetPasswordMask(nsAString& aPasswordMask) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPasswordMask(aPasswordMask); } \
  NS_IMETHOD GetTextLength(int32_t *aTextLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTextLength(aTextLength); } \
  NS_IMETHOD GetWrapWidth(int32_t *aWrapWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWrapWidth(aWrapWidth); } \
  NS_IMETHOD SetWrapWidth(int32_t aWrapWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWrapWidth(aWrapWidth); } \
  NS_IMETHOD GetNewlineHandling(int32_t *aNewlineHandling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewlineHandling(aNewlineHandling); } \
  NS_IMETHOD SetNewlineHandling(int32_t aNewlineHandling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNewlineHandling(aNewlineHandling); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertText(const nsAString& aStringToInsert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertText(aStringToInsert); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD InsertLineBreak(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertLineBreak(); } \


#endif /* __gen_nsIEditor_h__ */
