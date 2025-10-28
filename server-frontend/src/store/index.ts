import { configureStore } from '@reduxjs/toolkit'
import authReducer from './authSlice'
import taskReducer from './taskSlice'
import projectReducer from './projectSlice'

export const store = configureStore({
  reducer: {
    auth: authReducer,
    task: taskReducer,
    project: projectReducer,
  },
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
