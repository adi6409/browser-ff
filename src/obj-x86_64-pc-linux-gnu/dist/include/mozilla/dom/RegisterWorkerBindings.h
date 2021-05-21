#ifndef mozilla_dom_RegisterWorkerBindings_h
#define mozilla_dom_RegisterWorkerBindings_h


namespace mozilla {
namespace dom {
bool
RegisterWorkerBindings(JSContext* aCx, JS::Handle<JSObject*> aObj);

} // namespace dom
} // namespace mozilla


#endif // mozilla_dom_RegisterWorkerBindings_h
