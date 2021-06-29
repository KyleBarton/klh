pub enum SessionState {
  //Brand new, no state whatsoever
  New,
  //After init
  PostInit,
  //Reserving here for a place to dynamically load code (if possible)
  PostHooks,
  UserReady,
  TearDownReady,
  //Must be the last SessionState
  PostTearDown,
}
