/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessible.idl
 */

#ifndef __gen_nsIAccessible_h__
#define __gen_nsIAccessible_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPersistentProperties; /* forward declaration */

class nsIAccessibleDocument; /* forward declaration */

class nsIAccessibleRelation; /* forward declaration */

class nsINode; /* webidl Node */

namespace mozilla {
namespace a11y {
class LocalAccessible;
}
}

/* starting interface:    nsIAccessible */
#define NS_IACCESSIBLE_IID_STR "de2869d9-563c-4943-996b-31a4daa4d097"

#define NS_IACCESSIBLE_IID \
  {0xde2869d9, 0x563c, 0x4943, \
    { 0x99, 0x6b, 0x31, 0xa4, 0xda, 0xa4, 0xd0, 0x97 }}

class nsIAccessible : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessible;

  /* readonly attribute nsIAccessible parent; */
  NS_IMETHOD GetParent(nsIAccessible **aParent) = 0;

  /* readonly attribute nsIAccessible nextSibling; */
  NS_IMETHOD GetNextSibling(nsIAccessible **aNextSibling) = 0;

  /* readonly attribute nsIAccessible previousSibling; */
  NS_IMETHOD GetPreviousSibling(nsIAccessible **aPreviousSibling) = 0;

  /* readonly attribute nsIAccessible firstChild; */
  NS_IMETHOD GetFirstChild(nsIAccessible **aFirstChild) = 0;

  /* readonly attribute nsIAccessible lastChild; */
  NS_IMETHOD GetLastChild(nsIAccessible **aLastChild) = 0;

  /* readonly attribute nsIArray children; */
  NS_IMETHOD GetChildren(nsIArray **aChildren) = 0;

  /* readonly attribute long childCount; */
  NS_IMETHOD GetChildCount(int32_t *aChildCount) = 0;

  /* readonly attribute long indexInParent; */
  NS_IMETHOD GetIndexInParent(int32_t *aIndexInParent) = 0;

  /* readonly attribute long long uniqueID; */
  NS_IMETHOD GetUniqueID(int64_t *aUniqueID) = 0;

  /* readonly attribute Node DOMNode; */
  NS_IMETHOD GetDOMNode(nsINode **aDOMNode) = 0;

  /* readonly attribute AString id; */
  NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute nsIAccessibleDocument document; */
  NS_IMETHOD GetDocument(nsIAccessibleDocument **aDocument) = 0;

  /* readonly attribute nsIAccessibleDocument rootDocument; */
  NS_IMETHOD GetRootDocument(nsIAccessibleDocument **aRootDocument) = 0;

  /* readonly attribute AString language; */
  NS_IMETHOD GetLanguage(nsAString& aLanguage) = 0;

  /* readonly attribute AString name; */
  NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString value; */
  NS_IMETHOD GetValue(nsAString& aValue) = 0;

  /* readonly attribute AString description; */
  NS_IMETHOD GetDescription(nsAString& aDescription) = 0;

  /* readonly attribute AString accessKey; */
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) = 0;

  /* readonly attribute AString keyboardShortcut; */
  NS_IMETHOD GetKeyboardShortcut(nsAString& aKeyboardShortcut) = 0;

  /* readonly attribute unsigned long role; */
  NS_IMETHOD GetRole(uint32_t *aRole) = 0;

  /* void getState (out unsigned long aState, out unsigned long aExtraState); */
  NS_IMETHOD GetState(uint32_t *aState, uint32_t *aExtraState) = 0;

  /* readonly attribute AString help; */
  NS_IMETHOD GetHelp(nsAString& aHelp) = 0;

  /* readonly attribute nsIAccessible focusedChild; */
  NS_IMETHOD GetFocusedChild(nsIAccessible **aFocusedChild) = 0;

  /* readonly attribute nsIPersistentProperties attributes; */
  NS_IMETHOD GetAttributes(nsIPersistentProperties **aAttributes) = 0;

  /* readonly attribute nsISupports nativeInterface; */
  NS_IMETHOD GetNativeInterface(nsISupports **aNativeInterface) = 0;

  /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
  NS_IMETHOD GroupPosition(int32_t *aGroupLevel, int32_t *aSimilarItemsInGroup, int32_t *aPositionInGroup) = 0;

  /* nsIAccessible getChildAtPoint (in long x, in long y); */
  NS_IMETHOD GetChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) = 0;

  /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
  NS_IMETHOD GetDeepestChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) = 0;

  /* nsIAccessible getDeepestChildAtPointInProcess (in long x, in long y); */
  NS_IMETHOD GetDeepestChildAtPointInProcess(int32_t x, int32_t y, nsIAccessible **_retval) = 0;

  /* nsIAccessible getChildAt (in long aChildIndex); */
  NS_IMETHOD GetChildAt(int32_t aChildIndex, nsIAccessible **_retval) = 0;

  /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
  NS_IMETHOD GetRelationByType(uint32_t aRelationType, nsIAccessibleRelation **_retval) = 0;

  /* nsIArray getRelations (); */
  NS_IMETHOD GetRelations(nsIArray **_retval) = 0;

  /* void getBounds (out long x, out long y, out long width, out long height); */
  NS_IMETHOD GetBounds(int32_t *x, int32_t *y, int32_t *width, int32_t *height) = 0;

  /* void getBoundsInCSSPixels (out long aX, out long aY, out long aWidth, out long aHeight); */
  NS_IMETHOD GetBoundsInCSSPixels(int32_t *aX, int32_t *aY, int32_t *aWidth, int32_t *aHeight) = 0;

  /* void setSelected (in boolean isSelected); */
  NS_IMETHOD SetSelected(bool isSelected) = 0;

  /* void takeSelection (); */
  NS_IMETHOD TakeSelection(void) = 0;

  /* void takeFocus (); */
  NS_IMETHOD TakeFocus(void) = 0;

  /* readonly attribute uint8_t actionCount; */
  NS_IMETHOD GetActionCount(uint8_t *aActionCount) = 0;

  /* AString getActionName (in uint8_t index); */
  NS_IMETHOD GetActionName(uint8_t index, nsAString& _retval) = 0;

  /* AString getActionDescription (in uint8_t aIndex); */
  NS_IMETHOD GetActionDescription(uint8_t aIndex, nsAString& _retval) = 0;

  /* void doAction (in uint8_t index); */
  NS_IMETHOD DoAction(uint8_t index) = 0;

  /* [can_run_script] void scrollTo (in unsigned long aScrollType); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ScrollTo(uint32_t aScrollType) = 0;

  /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
  NS_IMETHOD ScrollToPoint(uint32_t coordinateType, int32_t x, int32_t y) = 0;

  /* void announce (in AString announcement, in unsigned short priority); */
  NS_IMETHOD Announce(const nsAString& announcement, uint16_t priority) = 0;

   virtual mozilla::a11y::LocalAccessible* ToInternalAccessible() const = 0;
  };

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessible, NS_IACCESSIBLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLE \
  NS_IMETHOD GetParent(nsIAccessible **aParent) override; \
  NS_IMETHOD GetNextSibling(nsIAccessible **aNextSibling) override; \
  NS_IMETHOD GetPreviousSibling(nsIAccessible **aPreviousSibling) override; \
  NS_IMETHOD GetFirstChild(nsIAccessible **aFirstChild) override; \
  NS_IMETHOD GetLastChild(nsIAccessible **aLastChild) override; \
  NS_IMETHOD GetChildren(nsIArray **aChildren) override; \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override; \
  NS_IMETHOD GetIndexInParent(int32_t *aIndexInParent) override; \
  NS_IMETHOD GetUniqueID(int64_t *aUniqueID) override; \
  NS_IMETHOD GetDOMNode(nsINode **aDOMNode) override; \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetDocument(nsIAccessibleDocument **aDocument) override; \
  NS_IMETHOD GetRootDocument(nsIAccessibleDocument **aRootDocument) override; \
  NS_IMETHOD GetLanguage(nsAString& aLanguage) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetValue(nsAString& aValue) override; \
  NS_IMETHOD GetDescription(nsAString& aDescription) override; \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override; \
  NS_IMETHOD GetKeyboardShortcut(nsAString& aKeyboardShortcut) override; \
  NS_IMETHOD GetRole(uint32_t *aRole) override; \
  NS_IMETHOD GetState(uint32_t *aState, uint32_t *aExtraState) override; \
  NS_IMETHOD GetHelp(nsAString& aHelp) override; \
  NS_IMETHOD GetFocusedChild(nsIAccessible **aFocusedChild) override; \
  NS_IMETHOD GetAttributes(nsIPersistentProperties **aAttributes) override; \
  NS_IMETHOD GetNativeInterface(nsISupports **aNativeInterface) override; \
  NS_IMETHOD GroupPosition(int32_t *aGroupLevel, int32_t *aSimilarItemsInGroup, int32_t *aPositionInGroup) override; \
  NS_IMETHOD GetChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override; \
  NS_IMETHOD GetDeepestChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override; \
  NS_IMETHOD GetDeepestChildAtPointInProcess(int32_t x, int32_t y, nsIAccessible **_retval) override; \
  NS_IMETHOD GetChildAt(int32_t aChildIndex, nsIAccessible **_retval) override; \
  NS_IMETHOD GetRelationByType(uint32_t aRelationType, nsIAccessibleRelation **_retval) override; \
  NS_IMETHOD GetRelations(nsIArray **_retval) override; \
  NS_IMETHOD GetBounds(int32_t *x, int32_t *y, int32_t *width, int32_t *height) override; \
  NS_IMETHOD GetBoundsInCSSPixels(int32_t *aX, int32_t *aY, int32_t *aWidth, int32_t *aHeight) override; \
  NS_IMETHOD SetSelected(bool isSelected) override; \
  NS_IMETHOD TakeSelection(void) override; \
  NS_IMETHOD TakeFocus(void) override; \
  NS_IMETHOD GetActionCount(uint8_t *aActionCount) override; \
  NS_IMETHOD GetActionName(uint8_t index, nsAString& _retval) override; \
  NS_IMETHOD GetActionDescription(uint8_t aIndex, nsAString& _retval) override; \
  NS_IMETHOD DoAction(uint8_t index) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ScrollTo(uint32_t aScrollType) override; \
  NS_IMETHOD ScrollToPoint(uint32_t coordinateType, int32_t x, int32_t y) override; \
  NS_IMETHOD Announce(const nsAString& announcement, uint16_t priority) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLE \
  nsresult GetParent(nsIAccessible **aParent); \
  nsresult GetNextSibling(nsIAccessible **aNextSibling); \
  nsresult GetPreviousSibling(nsIAccessible **aPreviousSibling); \
  nsresult GetFirstChild(nsIAccessible **aFirstChild); \
  nsresult GetLastChild(nsIAccessible **aLastChild); \
  nsresult GetChildren(nsIArray **aChildren); \
  nsresult GetChildCount(int32_t *aChildCount); \
  nsresult GetIndexInParent(int32_t *aIndexInParent); \
  nsresult GetUniqueID(int64_t *aUniqueID); \
  nsresult GetDOMNode(nsINode **aDOMNode); \
  nsresult GetId(nsAString& aId); \
  nsresult GetDocument(nsIAccessibleDocument **aDocument); \
  nsresult GetRootDocument(nsIAccessibleDocument **aRootDocument); \
  nsresult GetLanguage(nsAString& aLanguage); \
  nsresult GetName(nsAString& aName); \
  nsresult GetValue(nsAString& aValue); \
  nsresult GetDescription(nsAString& aDescription); \
  nsresult GetAccessKey(nsAString& aAccessKey); \
  nsresult GetKeyboardShortcut(nsAString& aKeyboardShortcut); \
  nsresult GetRole(uint32_t *aRole); \
  nsresult GetState(uint32_t *aState, uint32_t *aExtraState); \
  nsresult GetHelp(nsAString& aHelp); \
  nsresult GetFocusedChild(nsIAccessible **aFocusedChild); \
  nsresult GetAttributes(nsIPersistentProperties **aAttributes); \
  nsresult GetNativeInterface(nsISupports **aNativeInterface); \
  nsresult GroupPosition(int32_t *aGroupLevel, int32_t *aSimilarItemsInGroup, int32_t *aPositionInGroup); \
  nsresult GetChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval); \
  nsresult GetDeepestChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval); \
  nsresult GetDeepestChildAtPointInProcess(int32_t x, int32_t y, nsIAccessible **_retval); \
  nsresult GetChildAt(int32_t aChildIndex, nsIAccessible **_retval); \
  nsresult GetRelationByType(uint32_t aRelationType, nsIAccessibleRelation **_retval); \
  nsresult GetRelations(nsIArray **_retval); \
  nsresult GetBounds(int32_t *x, int32_t *y, int32_t *width, int32_t *height); \
  nsresult GetBoundsInCSSPixels(int32_t *aX, int32_t *aY, int32_t *aWidth, int32_t *aHeight); \
  nsresult SetSelected(bool isSelected); \
  nsresult TakeSelection(void); \
  nsresult TakeFocus(void); \
  nsresult GetActionCount(uint8_t *aActionCount); \
  nsresult GetActionName(uint8_t index, nsAString& _retval); \
  nsresult GetActionDescription(uint8_t aIndex, nsAString& _retval); \
  nsresult DoAction(uint8_t index); \
  MOZ_CAN_RUN_SCRIPT nsresult ScrollTo(uint32_t aScrollType); \
  nsresult ScrollToPoint(uint32_t coordinateType, int32_t x, int32_t y); \
  nsresult Announce(const nsAString& announcement, uint16_t priority); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLE(_to) \
  NS_IMETHOD GetParent(nsIAccessible **aParent) override { return _to GetParent(aParent); } \
  NS_IMETHOD GetNextSibling(nsIAccessible **aNextSibling) override { return _to GetNextSibling(aNextSibling); } \
  NS_IMETHOD GetPreviousSibling(nsIAccessible **aPreviousSibling) override { return _to GetPreviousSibling(aPreviousSibling); } \
  NS_IMETHOD GetFirstChild(nsIAccessible **aFirstChild) override { return _to GetFirstChild(aFirstChild); } \
  NS_IMETHOD GetLastChild(nsIAccessible **aLastChild) override { return _to GetLastChild(aLastChild); } \
  NS_IMETHOD GetChildren(nsIArray **aChildren) override { return _to GetChildren(aChildren); } \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override { return _to GetChildCount(aChildCount); } \
  NS_IMETHOD GetIndexInParent(int32_t *aIndexInParent) override { return _to GetIndexInParent(aIndexInParent); } \
  NS_IMETHOD GetUniqueID(int64_t *aUniqueID) override { return _to GetUniqueID(aUniqueID); } \
  NS_IMETHOD GetDOMNode(nsINode **aDOMNode) override { return _to GetDOMNode(aDOMNode); } \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetDocument(nsIAccessibleDocument **aDocument) override { return _to GetDocument(aDocument); } \
  NS_IMETHOD GetRootDocument(nsIAccessibleDocument **aRootDocument) override { return _to GetRootDocument(aRootDocument); } \
  NS_IMETHOD GetLanguage(nsAString& aLanguage) override { return _to GetLanguage(aLanguage); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return _to GetValue(aValue); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override { return _to GetAccessKey(aAccessKey); } \
  NS_IMETHOD GetKeyboardShortcut(nsAString& aKeyboardShortcut) override { return _to GetKeyboardShortcut(aKeyboardShortcut); } \
  NS_IMETHOD GetRole(uint32_t *aRole) override { return _to GetRole(aRole); } \
  NS_IMETHOD GetState(uint32_t *aState, uint32_t *aExtraState) override { return _to GetState(aState, aExtraState); } \
  NS_IMETHOD GetHelp(nsAString& aHelp) override { return _to GetHelp(aHelp); } \
  NS_IMETHOD GetFocusedChild(nsIAccessible **aFocusedChild) override { return _to GetFocusedChild(aFocusedChild); } \
  NS_IMETHOD GetAttributes(nsIPersistentProperties **aAttributes) override { return _to GetAttributes(aAttributes); } \
  NS_IMETHOD GetNativeInterface(nsISupports **aNativeInterface) override { return _to GetNativeInterface(aNativeInterface); } \
  NS_IMETHOD GroupPosition(int32_t *aGroupLevel, int32_t *aSimilarItemsInGroup, int32_t *aPositionInGroup) override { return _to GroupPosition(aGroupLevel, aSimilarItemsInGroup, aPositionInGroup); } \
  NS_IMETHOD GetChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override { return _to GetChildAtPoint(x, y, _retval); } \
  NS_IMETHOD GetDeepestChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override { return _to GetDeepestChildAtPoint(x, y, _retval); } \
  NS_IMETHOD GetDeepestChildAtPointInProcess(int32_t x, int32_t y, nsIAccessible **_retval) override { return _to GetDeepestChildAtPointInProcess(x, y, _retval); } \
  NS_IMETHOD GetChildAt(int32_t aChildIndex, nsIAccessible **_retval) override { return _to GetChildAt(aChildIndex, _retval); } \
  NS_IMETHOD GetRelationByType(uint32_t aRelationType, nsIAccessibleRelation **_retval) override { return _to GetRelationByType(aRelationType, _retval); } \
  NS_IMETHOD GetRelations(nsIArray **_retval) override { return _to GetRelations(_retval); } \
  NS_IMETHOD GetBounds(int32_t *x, int32_t *y, int32_t *width, int32_t *height) override { return _to GetBounds(x, y, width, height); } \
  NS_IMETHOD GetBoundsInCSSPixels(int32_t *aX, int32_t *aY, int32_t *aWidth, int32_t *aHeight) override { return _to GetBoundsInCSSPixels(aX, aY, aWidth, aHeight); } \
  NS_IMETHOD SetSelected(bool isSelected) override { return _to SetSelected(isSelected); } \
  NS_IMETHOD TakeSelection(void) override { return _to TakeSelection(); } \
  NS_IMETHOD TakeFocus(void) override { return _to TakeFocus(); } \
  NS_IMETHOD GetActionCount(uint8_t *aActionCount) override { return _to GetActionCount(aActionCount); } \
  NS_IMETHOD GetActionName(uint8_t index, nsAString& _retval) override { return _to GetActionName(index, _retval); } \
  NS_IMETHOD GetActionDescription(uint8_t aIndex, nsAString& _retval) override { return _to GetActionDescription(aIndex, _retval); } \
  NS_IMETHOD DoAction(uint8_t index) override { return _to DoAction(index); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ScrollTo(uint32_t aScrollType) override { return _to ScrollTo(aScrollType); } \
  NS_IMETHOD ScrollToPoint(uint32_t coordinateType, int32_t x, int32_t y) override { return _to ScrollToPoint(coordinateType, x, y); } \
  NS_IMETHOD Announce(const nsAString& announcement, uint16_t priority) override { return _to Announce(announcement, priority); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLE(_to) \
  NS_IMETHOD GetParent(nsIAccessible **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD GetNextSibling(nsIAccessible **aNextSibling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextSibling(aNextSibling); } \
  NS_IMETHOD GetPreviousSibling(nsIAccessible **aPreviousSibling) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreviousSibling(aPreviousSibling); } \
  NS_IMETHOD GetFirstChild(nsIAccessible **aFirstChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFirstChild(aFirstChild); } \
  NS_IMETHOD GetLastChild(nsIAccessible **aLastChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastChild(aLastChild); } \
  NS_IMETHOD GetChildren(nsIArray **aChildren) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildren(aChildren); } \
  NS_IMETHOD GetChildCount(int32_t *aChildCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildCount(aChildCount); } \
  NS_IMETHOD GetIndexInParent(int32_t *aIndexInParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIndexInParent(aIndexInParent); } \
  NS_IMETHOD GetUniqueID(int64_t *aUniqueID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUniqueID(aUniqueID); } \
  NS_IMETHOD GetDOMNode(nsINode **aDOMNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDOMNode(aDOMNode); } \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetDocument(nsIAccessibleDocument **aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocument(aDocument); } \
  NS_IMETHOD GetRootDocument(nsIAccessibleDocument **aRootDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootDocument(aRootDocument); } \
  NS_IMETHOD GetLanguage(nsAString& aLanguage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLanguage(aLanguage); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetValue(nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD GetAccessKey(nsAString& aAccessKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAccessKey(aAccessKey); } \
  NS_IMETHOD GetKeyboardShortcut(nsAString& aKeyboardShortcut) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyboardShortcut(aKeyboardShortcut); } \
  NS_IMETHOD GetRole(uint32_t *aRole) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRole(aRole); } \
  NS_IMETHOD GetState(uint32_t *aState, uint32_t *aExtraState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(aState, aExtraState); } \
  NS_IMETHOD GetHelp(nsAString& aHelp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHelp(aHelp); } \
  NS_IMETHOD GetFocusedChild(nsIAccessible **aFocusedChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedChild(aFocusedChild); } \
  NS_IMETHOD GetAttributes(nsIPersistentProperties **aAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAttributes(aAttributes); } \
  NS_IMETHOD GetNativeInterface(nsISupports **aNativeInterface) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNativeInterface(aNativeInterface); } \
  NS_IMETHOD GroupPosition(int32_t *aGroupLevel, int32_t *aSimilarItemsInGroup, int32_t *aPositionInGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GroupPosition(aGroupLevel, aSimilarItemsInGroup, aPositionInGroup); } \
  NS_IMETHOD GetChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildAtPoint(x, y, _retval); } \
  NS_IMETHOD GetDeepestChildAtPoint(int32_t x, int32_t y, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeepestChildAtPoint(x, y, _retval); } \
  NS_IMETHOD GetDeepestChildAtPointInProcess(int32_t x, int32_t y, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeepestChildAtPointInProcess(x, y, _retval); } \
  NS_IMETHOD GetChildAt(int32_t aChildIndex, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildAt(aChildIndex, _retval); } \
  NS_IMETHOD GetRelationByType(uint32_t aRelationType, nsIAccessibleRelation **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelationByType(aRelationType, _retval); } \
  NS_IMETHOD GetRelations(nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelations(_retval); } \
  NS_IMETHOD GetBounds(int32_t *x, int32_t *y, int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBounds(x, y, width, height); } \
  NS_IMETHOD GetBoundsInCSSPixels(int32_t *aX, int32_t *aY, int32_t *aWidth, int32_t *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBoundsInCSSPixels(aX, aY, aWidth, aHeight); } \
  NS_IMETHOD SetSelected(bool isSelected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelected(isSelected); } \
  NS_IMETHOD TakeSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TakeSelection(); } \
  NS_IMETHOD TakeFocus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TakeFocus(); } \
  NS_IMETHOD GetActionCount(uint8_t *aActionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActionCount(aActionCount); } \
  NS_IMETHOD GetActionName(uint8_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActionName(index, _retval); } \
  NS_IMETHOD GetActionDescription(uint8_t aIndex, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActionDescription(aIndex, _retval); } \
  NS_IMETHOD DoAction(uint8_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoAction(index); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ScrollTo(uint32_t aScrollType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollTo(aScrollType); } \
  NS_IMETHOD ScrollToPoint(uint32_t coordinateType, int32_t x, int32_t y) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollToPoint(coordinateType, x, y); } \
  NS_IMETHOD Announce(const nsAString& announcement, uint16_t priority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Announce(announcement, priority); } \


#endif /* __gen_nsIAccessible_h__ */
