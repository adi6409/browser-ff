/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionController.idl
 */

#ifndef __gen_nsISelectionController_h__
#define __gen_nsISelectionController_h__


#ifndef __gen_nsISelectionDisplay_h__
#include "nsISelectionDisplay.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
typedef short SelectionRegion;
namespace mozilla {
namespace dom {
class Selection;
} // namespace dom
} // namespace mozilla
class nsIContent; /* forward declaration */

class nsISelectionDisplay; /* forward declaration */

class nsINode; /* webidl Node */

namespace mozilla {
namespace dom {
class Selection; /* webidl Selection */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsISelectionController */
#define NS_ISELECTIONCONTROLLER_IID_STR "3801c9d4-8e69-4bfc-9edb-b58278621f8f"

#define NS_ISELECTIONCONTROLLER_IID \
  {0x3801c9d4, 0x8e69, 0x4bfc, \
    { 0x9e, 0xdb, 0xb5, 0x82, 0x78, 0x62, 0x1f, 0x8f }}

class NS_NO_VTABLE nsISelectionController : public nsISelectionDisplay {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISELECTIONCONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISelectionController;

  enum {
    SELECTION_NONE = 0,
    SELECTION_NORMAL = 1,
    SELECTION_SPELLCHECK = 2,
    SELECTION_IME_RAWINPUT = 3,
    SELECTION_IME_SELECTEDRAWTEXT = 4,
    SELECTION_IME_CONVERTEDTEXT = 5,
    SELECTION_IME_SELECTEDCONVERTEDTEXT = 6,
    SELECTION_ACCESSIBILITY = 7,
    SELECTION_FIND = 8,
    SELECTION_URLSECONDARY = 9,
    SELECTION_URLSTRIKEOUT = 10,
    NUM_SELECTIONTYPES = 11,
    SELECTION_ANCHOR_REGION = 0,
    SELECTION_FOCUS_REGION = 1,
    SELECTION_WHOLE_SELECTION = 2,
    NUM_SELECTION_REGIONS = 3,
    SELECTION_OFF = 0,
    SELECTION_HIDDEN = 1,
    SELECTION_ON = 2,
    SELECTION_DISABLED = 3,
    SELECTION_ATTENTION = 4
  };

  /* void setDisplaySelection (in short toggle); */
  NS_IMETHOD SetDisplaySelection(int16_t toggle) = 0;

  /* short getDisplaySelection (); */
  NS_IMETHOD GetDisplaySelection(int16_t *_retval) = 0;

  /* [binaryname(GetSelectionFromScript)] Selection getSelection (in short type); */
  NS_IMETHOD GetSelectionFromScript(int16_t type, mozilla::dom::Selection **_retval) = 0;

  /* [binaryname(GetSelection),noscript,nostdcall,notxpcom] Selection getDOMSelection (in short aType); */
  virtual mozilla::dom::Selection * GetSelection(int16_t aType) = 0;

  /* [noscript,nostdcall,notxpcom] void selectionWillTakeFocus (); */
  virtual void SelectionWillTakeFocus(void) = 0;

  /* [noscript,nostdcall,notxpcom] void selectionWillLoseFocus (); */
  virtual void SelectionWillLoseFocus(void) = 0;

  enum {
    SCROLL_SYNCHRONOUS = 2,
    SCROLL_FIRST_ANCESTOR_ONLY = 4,
    SCROLL_CENTER_VERTICALLY = 16,
    SCROLL_OVERFLOW_HIDDEN = 32,
    SCROLL_FOR_CARET_MOVE = 64
  };

  /* void scrollSelectionIntoView (in short type, in short region, in short flags); */
  NS_IMETHOD ScrollSelectionIntoView(int16_t type, int16_t region, int16_t flags) = 0;

  /* void repaintSelection (in short type); */
  NS_IMETHOD RepaintSelection(int16_t type) = 0;

  /* void setCaretEnabled (in boolean enabled); */
  NS_IMETHOD SetCaretEnabled(bool enabled) = 0;

  /* void setCaretReadOnly (in boolean readOnly); */
  NS_IMETHOD SetCaretReadOnly(bool readOnly) = 0;

  /* boolean getCaretEnabled (); */
  NS_IMETHOD GetCaretEnabled(bool *_retval) = 0;

  /* readonly attribute boolean caretVisible; */
  NS_IMETHOD GetCaretVisible(bool *aCaretVisible) = 0;

  /* void setCaretVisibilityDuringSelection (in boolean visibility); */
  NS_IMETHOD SetCaretVisibilityDuringSelection(bool visibility) = 0;

  /* void characterMove (in boolean forward, in boolean extend); */
  NS_IMETHOD CharacterMove(bool forward, bool extend) = 0;

  /* void physicalMove (in short direction, in short amount, in boolean extend); */
  NS_IMETHOD PhysicalMove(int16_t direction, int16_t amount, bool extend) = 0;

  enum {
    MOVE_LEFT = 0,
    MOVE_RIGHT = 1,
    MOVE_UP = 2,
    MOVE_DOWN = 3
  };

  /* void wordMove (in boolean forward, in boolean extend); */
  NS_IMETHOD WordMove(bool forward, bool extend) = 0;

  /* void lineMove (in boolean forward, in boolean extend); */
  NS_IMETHOD LineMove(bool forward, bool extend) = 0;

  /* void intraLineMove (in boolean forward, in boolean extend); */
  NS_IMETHOD IntraLineMove(bool forward, bool extend) = 0;

  /* [can_run_script] void pageMove (in boolean forward, in boolean extend); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PageMove(bool forward, bool extend) = 0;

  /* void completeScroll (in boolean forward); */
  NS_IMETHOD CompleteScroll(bool forward) = 0;

  /* [can_run_script] void completeMove (in boolean forward, in boolean extend); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteMove(bool forward, bool extend) = 0;

  /* void scrollPage (in boolean forward); */
  NS_IMETHOD ScrollPage(bool forward) = 0;

  /* void scrollLine (in boolean forward); */
  NS_IMETHOD ScrollLine(bool forward) = 0;

  /* void scrollCharacter (in boolean right); */
  NS_IMETHOD ScrollCharacter(bool right) = 0;

  /* boolean checkVisibility (in Node node, in short startOffset, in short endOffset); */
  NS_IMETHOD CheckVisibility(nsINode *node, int16_t startOffset, int16_t endOffset, bool *_retval) = 0;

  /* [noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset); */
  virtual nsresult CheckVisibilityContent(nsIContent *node, int16_t startOffset, int16_t endOffset, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISelectionController, NS_ISELECTIONCONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISELECTIONCONTROLLER \
  NS_IMETHOD SetDisplaySelection(int16_t toggle) override; \
  NS_IMETHOD GetDisplaySelection(int16_t *_retval) override; \
  NS_IMETHOD GetSelectionFromScript(int16_t type, mozilla::dom::Selection **_retval) override; \
  virtual mozilla::dom::Selection * GetSelection(int16_t aType) override; \
  virtual void SelectionWillTakeFocus(void) override; \
  virtual void SelectionWillLoseFocus(void) override; \
  NS_IMETHOD ScrollSelectionIntoView(int16_t type, int16_t region, int16_t flags) override; \
  NS_IMETHOD RepaintSelection(int16_t type) override; \
  NS_IMETHOD SetCaretEnabled(bool enabled) override; \
  NS_IMETHOD SetCaretReadOnly(bool readOnly) override; \
  NS_IMETHOD GetCaretEnabled(bool *_retval) override; \
  NS_IMETHOD GetCaretVisible(bool *aCaretVisible) override; \
  NS_IMETHOD SetCaretVisibilityDuringSelection(bool visibility) override; \
  NS_IMETHOD CharacterMove(bool forward, bool extend) override; \
  NS_IMETHOD PhysicalMove(int16_t direction, int16_t amount, bool extend) override; \
  NS_IMETHOD WordMove(bool forward, bool extend) override; \
  NS_IMETHOD LineMove(bool forward, bool extend) override; \
  NS_IMETHOD IntraLineMove(bool forward, bool extend) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PageMove(bool forward, bool extend) override; \
  NS_IMETHOD CompleteScroll(bool forward) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteMove(bool forward, bool extend) override; \
  NS_IMETHOD ScrollPage(bool forward) override; \
  NS_IMETHOD ScrollLine(bool forward) override; \
  NS_IMETHOD ScrollCharacter(bool right) override; \
  NS_IMETHOD CheckVisibility(nsINode *node, int16_t startOffset, int16_t endOffset, bool *_retval) override; \
  virtual nsresult CheckVisibilityContent(nsIContent *node, int16_t startOffset, int16_t endOffset, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISELECTIONCONTROLLER \
  nsresult SetDisplaySelection(int16_t toggle); \
  nsresult GetDisplaySelection(int16_t *_retval); \
  nsresult GetSelectionFromScript(int16_t type, mozilla::dom::Selection **_retval); \
  mozilla::dom::Selection * GetSelection(int16_t aType); \
  void SelectionWillTakeFocus(void); \
  void SelectionWillLoseFocus(void); \
  nsresult ScrollSelectionIntoView(int16_t type, int16_t region, int16_t flags); \
  nsresult RepaintSelection(int16_t type); \
  nsresult SetCaretEnabled(bool enabled); \
  nsresult SetCaretReadOnly(bool readOnly); \
  nsresult GetCaretEnabled(bool *_retval); \
  nsresult GetCaretVisible(bool *aCaretVisible); \
  nsresult SetCaretVisibilityDuringSelection(bool visibility); \
  nsresult CharacterMove(bool forward, bool extend); \
  nsresult PhysicalMove(int16_t direction, int16_t amount, bool extend); \
  nsresult WordMove(bool forward, bool extend); \
  nsresult LineMove(bool forward, bool extend); \
  nsresult IntraLineMove(bool forward, bool extend); \
  MOZ_CAN_RUN_SCRIPT nsresult PageMove(bool forward, bool extend); \
  nsresult CompleteScroll(bool forward); \
  MOZ_CAN_RUN_SCRIPT nsresult CompleteMove(bool forward, bool extend); \
  nsresult ScrollPage(bool forward); \
  nsresult ScrollLine(bool forward); \
  nsresult ScrollCharacter(bool right); \
  nsresult CheckVisibility(nsINode *node, int16_t startOffset, int16_t endOffset, bool *_retval); \
  nsresult CheckVisibilityContent(nsIContent *node, int16_t startOffset, int16_t endOffset, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISELECTIONCONTROLLER(_to) \
  NS_IMETHOD SetDisplaySelection(int16_t toggle) override { return _to SetDisplaySelection(toggle); } \
  NS_IMETHOD GetDisplaySelection(int16_t *_retval) override { return _to GetDisplaySelection(_retval); } \
  NS_IMETHOD GetSelectionFromScript(int16_t type, mozilla::dom::Selection **_retval) override { return _to GetSelectionFromScript(type, _retval); } \
  virtual mozilla::dom::Selection * GetSelection(int16_t aType) override { return _to GetSelection(aType); } \
  virtual void SelectionWillTakeFocus(void) override { return _to SelectionWillTakeFocus(); } \
  virtual void SelectionWillLoseFocus(void) override { return _to SelectionWillLoseFocus(); } \
  NS_IMETHOD ScrollSelectionIntoView(int16_t type, int16_t region, int16_t flags) override { return _to ScrollSelectionIntoView(type, region, flags); } \
  NS_IMETHOD RepaintSelection(int16_t type) override { return _to RepaintSelection(type); } \
  NS_IMETHOD SetCaretEnabled(bool enabled) override { return _to SetCaretEnabled(enabled); } \
  NS_IMETHOD SetCaretReadOnly(bool readOnly) override { return _to SetCaretReadOnly(readOnly); } \
  NS_IMETHOD GetCaretEnabled(bool *_retval) override { return _to GetCaretEnabled(_retval); } \
  NS_IMETHOD GetCaretVisible(bool *aCaretVisible) override { return _to GetCaretVisible(aCaretVisible); } \
  NS_IMETHOD SetCaretVisibilityDuringSelection(bool visibility) override { return _to SetCaretVisibilityDuringSelection(visibility); } \
  NS_IMETHOD CharacterMove(bool forward, bool extend) override { return _to CharacterMove(forward, extend); } \
  NS_IMETHOD PhysicalMove(int16_t direction, int16_t amount, bool extend) override { return _to PhysicalMove(direction, amount, extend); } \
  NS_IMETHOD WordMove(bool forward, bool extend) override { return _to WordMove(forward, extend); } \
  NS_IMETHOD LineMove(bool forward, bool extend) override { return _to LineMove(forward, extend); } \
  NS_IMETHOD IntraLineMove(bool forward, bool extend) override { return _to IntraLineMove(forward, extend); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PageMove(bool forward, bool extend) override { return _to PageMove(forward, extend); } \
  NS_IMETHOD CompleteScroll(bool forward) override { return _to CompleteScroll(forward); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteMove(bool forward, bool extend) override { return _to CompleteMove(forward, extend); } \
  NS_IMETHOD ScrollPage(bool forward) override { return _to ScrollPage(forward); } \
  NS_IMETHOD ScrollLine(bool forward) override { return _to ScrollLine(forward); } \
  NS_IMETHOD ScrollCharacter(bool right) override { return _to ScrollCharacter(right); } \
  NS_IMETHOD CheckVisibility(nsINode *node, int16_t startOffset, int16_t endOffset, bool *_retval) override { return _to CheckVisibility(node, startOffset, endOffset, _retval); } \
  virtual nsresult CheckVisibilityContent(nsIContent *node, int16_t startOffset, int16_t endOffset, bool *_retval) override { return _to CheckVisibilityContent(node, startOffset, endOffset, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISELECTIONCONTROLLER(_to) \
  NS_IMETHOD SetDisplaySelection(int16_t toggle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplaySelection(toggle); } \
  NS_IMETHOD GetDisplaySelection(int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplaySelection(_retval); } \
  NS_IMETHOD GetSelectionFromScript(int16_t type, mozilla::dom::Selection **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionFromScript(type, _retval); } \
  virtual mozilla::dom::Selection * GetSelection(int16_t aType) override; \
  virtual void SelectionWillTakeFocus(void) override; \
  virtual void SelectionWillLoseFocus(void) override; \
  NS_IMETHOD ScrollSelectionIntoView(int16_t type, int16_t region, int16_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollSelectionIntoView(type, region, flags); } \
  NS_IMETHOD RepaintSelection(int16_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RepaintSelection(type); } \
  NS_IMETHOD SetCaretEnabled(bool enabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaretEnabled(enabled); } \
  NS_IMETHOD SetCaretReadOnly(bool readOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaretReadOnly(readOnly); } \
  NS_IMETHOD GetCaretEnabled(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCaretEnabled(_retval); } \
  NS_IMETHOD GetCaretVisible(bool *aCaretVisible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCaretVisible(aCaretVisible); } \
  NS_IMETHOD SetCaretVisibilityDuringSelection(bool visibility) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaretVisibilityDuringSelection(visibility); } \
  NS_IMETHOD CharacterMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CharacterMove(forward, extend); } \
  NS_IMETHOD PhysicalMove(int16_t direction, int16_t amount, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PhysicalMove(direction, amount, extend); } \
  NS_IMETHOD WordMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WordMove(forward, extend); } \
  NS_IMETHOD LineMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LineMove(forward, extend); } \
  NS_IMETHOD IntraLineMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IntraLineMove(forward, extend); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD PageMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PageMove(forward, extend); } \
  NS_IMETHOD CompleteScroll(bool forward) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompleteScroll(forward); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteMove(bool forward, bool extend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompleteMove(forward, extend); } \
  NS_IMETHOD ScrollPage(bool forward) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollPage(forward); } \
  NS_IMETHOD ScrollLine(bool forward) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollLine(forward); } \
  NS_IMETHOD ScrollCharacter(bool right) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollCharacter(right); } \
  NS_IMETHOD CheckVisibility(nsINode *node, int16_t startOffset, int16_t endOffset, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckVisibility(node, startOffset, endOffset, _retval); } \
  virtual nsresult CheckVisibilityContent(nsIContent *node, int16_t startOffset, int16_t endOffset, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckVisibilityContent(node, startOffset, endOffset, _retval); } 

   #define NS_ISELECTIONCONTROLLER_CID \
   { 0x513b9460, 0xd56a, 0x4c4e, \
   { 0xb6, 0xf9, 0x0b, 0x8a, 0xe4, 0x37, 0x2a, 0x3b }}
namespace mozilla {
// RawSelectionType should be used to store nsISelectionController::SELECTION_*.
typedef short RawSelectionType;
// SelectionTypeMask should be used to store bit-mask of selection types.
// The value can be retrieved with ToSelectionTypeMask() and checking if
// a selection type is in a mask with |SelectionType & SelectionTypeMask|.
typedef uint16_t SelectionTypeMask;
// SelectionType should be used in internal handling because of type safe.
enum class SelectionType : RawSelectionType
{
  eInvalid = -1,
  eNone = nsISelectionController::SELECTION_NONE,
  eNormal = nsISelectionController::SELECTION_NORMAL,
  eSpellCheck = nsISelectionController::SELECTION_SPELLCHECK,
  eIMERawClause = nsISelectionController::SELECTION_IME_RAWINPUT,
  eIMESelectedRawClause = nsISelectionController::SELECTION_IME_SELECTEDRAWTEXT,
  eIMEConvertedClause = nsISelectionController::SELECTION_IME_CONVERTEDTEXT,
  eIMESelectedClause =
    nsISelectionController::SELECTION_IME_SELECTEDCONVERTEDTEXT,
  eAccessibility = nsISelectionController::SELECTION_ACCESSIBILITY,
  eFind = nsISelectionController::SELECTION_FIND,
  eURLSecondary = nsISelectionController::SELECTION_URLSECONDARY,
  eURLStrikeout = nsISelectionController::SELECTION_URLSTRIKEOUT,
};
// Using anonymous enum to define constants because these constants may be
// used at defining fixed size array in some header files (e.g.,
// nsFrameSelection.h).  So, the values needs to be defined here, but we cannot
// use static/const even with extern since it causes failing to link or
// initializes them after such headers.
enum : size_t
{
  // kSelectionTypeCount is number of SelectionType.
  kSelectionTypeCount = nsISelectionController::NUM_SELECTIONTYPES,
};
// kPresentSelectionTypes is selection types which may be displayed.
// I.e., selection types except eNone.
static const SelectionType kPresentSelectionTypes[] = {
  SelectionType::eNormal,
  SelectionType::eSpellCheck,
  SelectionType::eIMERawClause,
  SelectionType::eIMESelectedRawClause,
  SelectionType::eIMEConvertedClause,
  SelectionType::eIMESelectedClause,
  SelectionType::eAccessibility,
  SelectionType::eFind,
  SelectionType::eURLSecondary,
  SelectionType::eURLStrikeout,
};
// Please include mozilla/dom/Selection.h for the following APIs.
const char* ToChar(SelectionType aSelectionType);
inline bool IsValidRawSelectionType(RawSelectionType aRawSelectionType);
inline SelectionType ToSelectionType(RawSelectionType aRawSelectionType);
inline RawSelectionType ToRawSelectionType(SelectionType aSelectionType);
inline SelectionTypeMask ToSelectionTypeMask(SelectionType aSelectionType);
} // namespace mozilla

#endif /* __gen_nsISelectionController_h__ */
