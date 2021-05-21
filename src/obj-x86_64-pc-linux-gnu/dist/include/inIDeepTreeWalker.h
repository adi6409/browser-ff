/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/inspector/inIDeepTreeWalker.idl
 */

#ifndef __gen_inIDeepTreeWalker_h__
#define __gen_inIDeepTreeWalker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsINode; /* webidl Node */


/* starting interface:    inIDeepTreeWalker */
#define INIDEEPTREEWALKER_IID_STR "6657e8eb-b646-48e7-993e-cfa6e96415b4"

#define INIDEEPTREEWALKER_IID \
  {0x6657e8eb, 0xb646, 0x48e7, \
    { 0x99, 0x3e, 0xcf, 0xa6, 0xe9, 0x64, 0x15, 0xb4 }}

class NS_NO_VTABLE inIDeepTreeWalker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(INIDEEPTREEWALKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = inIDeepTreeWalker;

  /* attribute boolean showAnonymousContent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShowAnonymousContent(bool *aShowAnonymousContent) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetShowAnonymousContent(bool aShowAnonymousContent) = 0;

  /* attribute boolean showSubDocuments; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShowSubDocuments(bool *aShowSubDocuments) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetShowSubDocuments(bool aShowSubDocuments) = 0;

  /* attribute boolean showDocumentsAsNodes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetShowDocumentsAsNodes(bool *aShowDocumentsAsNodes) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetShowDocumentsAsNodes(bool aShowDocumentsAsNodes) = 0;

  /* void init (in Node aRoot, in unsigned long aWhatToShow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsINode *aRoot, uint32_t aWhatToShow) = 0;

  /* readonly attribute Node root; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRoot(nsINode **aRoot) = 0;

  /* readonly attribute unsigned long whatToShow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWhatToShow(uint32_t *aWhatToShow) = 0;

  /* attribute Node currentNode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentNode(nsINode *aCurrentNode) = 0;

  /* Node parentNode (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParentNode(nsINode **_retval) = 0;

  /* Node firstChild (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FirstChild(nsINode **_retval) = 0;

  /* Node lastChild (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LastChild(nsINode **_retval) = 0;

  /* Node previousSibling (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreviousSibling(nsINode **_retval) = 0;

  /* Node nextSibling (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NextSibling(nsINode **_retval) = 0;

  /* Node previousNode (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PreviousNode(nsINode **_retval) = 0;

  /* Node nextNode (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NextNode(nsINode **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(inIDeepTreeWalker, INIDEEPTREEWALKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_INIDEEPTREEWALKER \
  NS_IMETHOD GetShowAnonymousContent(bool *aShowAnonymousContent) override; \
  NS_IMETHOD SetShowAnonymousContent(bool aShowAnonymousContent) override; \
  NS_IMETHOD GetShowSubDocuments(bool *aShowSubDocuments) override; \
  NS_IMETHOD SetShowSubDocuments(bool aShowSubDocuments) override; \
  NS_IMETHOD GetShowDocumentsAsNodes(bool *aShowDocumentsAsNodes) override; \
  NS_IMETHOD SetShowDocumentsAsNodes(bool aShowDocumentsAsNodes) override; \
  NS_IMETHOD Init(nsINode *aRoot, uint32_t aWhatToShow) override; \
  NS_IMETHOD GetRoot(nsINode **aRoot) override; \
  NS_IMETHOD GetWhatToShow(uint32_t *aWhatToShow) override; \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override; \
  NS_IMETHOD SetCurrentNode(nsINode *aCurrentNode) override; \
  NS_IMETHOD ParentNode(nsINode **_retval) override; \
  NS_IMETHOD FirstChild(nsINode **_retval) override; \
  NS_IMETHOD LastChild(nsINode **_retval) override; \
  NS_IMETHOD PreviousSibling(nsINode **_retval) override; \
  NS_IMETHOD NextSibling(nsINode **_retval) override; \
  NS_IMETHOD PreviousNode(nsINode **_retval) override; \
  NS_IMETHOD NextNode(nsINode **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_INIDEEPTREEWALKER \
  nsresult GetShowAnonymousContent(bool *aShowAnonymousContent); \
  nsresult SetShowAnonymousContent(bool aShowAnonymousContent); \
  nsresult GetShowSubDocuments(bool *aShowSubDocuments); \
  nsresult SetShowSubDocuments(bool aShowSubDocuments); \
  nsresult GetShowDocumentsAsNodes(bool *aShowDocumentsAsNodes); \
  nsresult SetShowDocumentsAsNodes(bool aShowDocumentsAsNodes); \
  nsresult Init(nsINode *aRoot, uint32_t aWhatToShow); \
  nsresult GetRoot(nsINode **aRoot); \
  nsresult GetWhatToShow(uint32_t *aWhatToShow); \
  nsresult GetCurrentNode(nsINode **aCurrentNode); \
  nsresult SetCurrentNode(nsINode *aCurrentNode); \
  nsresult ParentNode(nsINode **_retval); \
  nsresult FirstChild(nsINode **_retval); \
  nsresult LastChild(nsINode **_retval); \
  nsresult PreviousSibling(nsINode **_retval); \
  nsresult NextSibling(nsINode **_retval); \
  nsresult PreviousNode(nsINode **_retval); \
  nsresult NextNode(nsINode **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_INIDEEPTREEWALKER(_to) \
  NS_IMETHOD GetShowAnonymousContent(bool *aShowAnonymousContent) override { return _to GetShowAnonymousContent(aShowAnonymousContent); } \
  NS_IMETHOD SetShowAnonymousContent(bool aShowAnonymousContent) override { return _to SetShowAnonymousContent(aShowAnonymousContent); } \
  NS_IMETHOD GetShowSubDocuments(bool *aShowSubDocuments) override { return _to GetShowSubDocuments(aShowSubDocuments); } \
  NS_IMETHOD SetShowSubDocuments(bool aShowSubDocuments) override { return _to SetShowSubDocuments(aShowSubDocuments); } \
  NS_IMETHOD GetShowDocumentsAsNodes(bool *aShowDocumentsAsNodes) override { return _to GetShowDocumentsAsNodes(aShowDocumentsAsNodes); } \
  NS_IMETHOD SetShowDocumentsAsNodes(bool aShowDocumentsAsNodes) override { return _to SetShowDocumentsAsNodes(aShowDocumentsAsNodes); } \
  NS_IMETHOD Init(nsINode *aRoot, uint32_t aWhatToShow) override { return _to Init(aRoot, aWhatToShow); } \
  NS_IMETHOD GetRoot(nsINode **aRoot) override { return _to GetRoot(aRoot); } \
  NS_IMETHOD GetWhatToShow(uint32_t *aWhatToShow) override { return _to GetWhatToShow(aWhatToShow); } \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override { return _to GetCurrentNode(aCurrentNode); } \
  NS_IMETHOD SetCurrentNode(nsINode *aCurrentNode) override { return _to SetCurrentNode(aCurrentNode); } \
  NS_IMETHOD ParentNode(nsINode **_retval) override { return _to ParentNode(_retval); } \
  NS_IMETHOD FirstChild(nsINode **_retval) override { return _to FirstChild(_retval); } \
  NS_IMETHOD LastChild(nsINode **_retval) override { return _to LastChild(_retval); } \
  NS_IMETHOD PreviousSibling(nsINode **_retval) override { return _to PreviousSibling(_retval); } \
  NS_IMETHOD NextSibling(nsINode **_retval) override { return _to NextSibling(_retval); } \
  NS_IMETHOD PreviousNode(nsINode **_retval) override { return _to PreviousNode(_retval); } \
  NS_IMETHOD NextNode(nsINode **_retval) override { return _to NextNode(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_INIDEEPTREEWALKER(_to) \
  NS_IMETHOD GetShowAnonymousContent(bool *aShowAnonymousContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShowAnonymousContent(aShowAnonymousContent); } \
  NS_IMETHOD SetShowAnonymousContent(bool aShowAnonymousContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShowAnonymousContent(aShowAnonymousContent); } \
  NS_IMETHOD GetShowSubDocuments(bool *aShowSubDocuments) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShowSubDocuments(aShowSubDocuments); } \
  NS_IMETHOD SetShowSubDocuments(bool aShowSubDocuments) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShowSubDocuments(aShowSubDocuments); } \
  NS_IMETHOD GetShowDocumentsAsNodes(bool *aShowDocumentsAsNodes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetShowDocumentsAsNodes(aShowDocumentsAsNodes); } \
  NS_IMETHOD SetShowDocumentsAsNodes(bool aShowDocumentsAsNodes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetShowDocumentsAsNodes(aShowDocumentsAsNodes); } \
  NS_IMETHOD Init(nsINode *aRoot, uint32_t aWhatToShow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aRoot, aWhatToShow); } \
  NS_IMETHOD GetRoot(nsINode **aRoot) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRoot(aRoot); } \
  NS_IMETHOD GetWhatToShow(uint32_t *aWhatToShow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWhatToShow(aWhatToShow); } \
  NS_IMETHOD GetCurrentNode(nsINode **aCurrentNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentNode(aCurrentNode); } \
  NS_IMETHOD SetCurrentNode(nsINode *aCurrentNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentNode(aCurrentNode); } \
  NS_IMETHOD ParentNode(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParentNode(_retval); } \
  NS_IMETHOD FirstChild(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FirstChild(_retval); } \
  NS_IMETHOD LastChild(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LastChild(_retval); } \
  NS_IMETHOD PreviousSibling(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreviousSibling(_retval); } \
  NS_IMETHOD NextSibling(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NextSibling(_retval); } \
  NS_IMETHOD PreviousNode(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreviousNode(_retval); } \
  NS_IMETHOD NextNode(nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NextNode(_retval); } 


#endif /* __gen_inIDeepTreeWalker_h__ */
