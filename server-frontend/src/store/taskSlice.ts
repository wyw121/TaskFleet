/**
 * TaskFleet - 任务管理Store
 */

import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import taskService from '../services/taskService';
import { Task, TaskInfo, CreateTaskRequest, UpdateTaskRequest, TaskQueryParams } from '../types';

interface TaskState {
  tasks: TaskInfo[];
  currentTask: TaskInfo | null;
  loading: boolean;
  error: string | null;
  filters: TaskQueryParams;
}

const initialState: TaskState = {
  tasks: [],
  currentTask: null,
  loading: false,
  error: null,
  filters: {},
};

// 异步Actions
export const fetchTasks = createAsyncThunk(
  'task/fetchTasks',
  async (params?: TaskQueryParams) => {
    const response = await taskService.getTasks(params);
    return response;
  }
);

export const fetchTask = createAsyncThunk(
  'task/fetchTask',
  async (id: string) => {
    const response = await taskService.getTask(id);
    return response;
  }
);

export const createTask = createAsyncThunk(
  'task/createTask',
  async (data: CreateTaskRequest) => {
    const response = await taskService.createTask(data);
    return response;
  }
);

export const updateTask = createAsyncThunk(
  'task/updateTask',
  async ({ id, data }: { id: string; data: UpdateTaskRequest }) => {
    const response = await taskService.updateTask(id, data);
    return response;
  }
);

export const deleteTask = createAsyncThunk(
  'task/deleteTask',
  async (id: string) => {
    await taskService.deleteTask(id);
    return id;
  }
);

export const startTask = createAsyncThunk(
  'task/startTask',
  async (id: string) => {
    const response = await taskService.startTask(id);
    return response;
  }
);

export const completeTask = createAsyncThunk(
  'task/completeTask',
  async (id: string) => {
    const response = await taskService.completeTask(id);
    return response;
  }
);

export const cancelTask = createAsyncThunk(
  'task/cancelTask',
  async (id: string) => {
    const response = await taskService.cancelTask(id);
    return response;
  }
);

export const assignTask = createAsyncThunk(
  'task/assignTask',
  async ({ id, userId }: { id: string; userId: string }) => {
    const response = await taskService.assignTask(id, { assigned_to: userId });
    return response;
  }
);

// Slice
const taskSlice = createSlice({
  name: 'task',
  initialState,
  reducers: {
    setFilters: (state, action: PayloadAction<TaskQueryParams>) => {
      state.filters = action.payload;
    },
    clearCurrentTask: (state) => {
      state.currentTask = null;
    },
    clearError: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    // fetchTasks
    builder.addCase(fetchTasks.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(fetchTasks.fulfilled, (state, action) => {
      state.loading = false;
      state.tasks = action.payload;
    });
    builder.addCase(fetchTasks.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to fetch tasks';
    });

    // fetchTask
    builder.addCase(fetchTask.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(fetchTask.fulfilled, (state, action) => {
      state.loading = false;
      state.currentTask = action.payload;
    });
    builder.addCase(fetchTask.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to fetch task';
    });

    // createTask
    builder.addCase(createTask.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(createTask.fulfilled, (state, action) => {
      state.loading = false;
      // 将新任务添加到列表
      state.tasks.unshift(action.payload as any);
    });
    builder.addCase(createTask.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to create task';
    });

    // updateTask
    builder.addCase(updateTask.fulfilled, (state, action) => {
      const index = state.tasks.findIndex(t => t.id === action.payload.id);
      if (index !== -1) {
        state.tasks[index] = action.payload as any;
      }
      if (state.currentTask?.id === action.payload.id) {
        state.currentTask = action.payload as any;
      }
    });

    // deleteTask
    builder.addCase(deleteTask.fulfilled, (state, action) => {
      state.tasks = state.tasks.filter(t => t.id !== action.payload);
      if (state.currentTask?.id === action.payload) {
        state.currentTask = null;
      }
    });

    // startTask, completeTask, cancelTask
    [startTask, completeTask, cancelTask].forEach(thunk => {
      builder.addCase(thunk.fulfilled, (state, action) => {
        const index = state.tasks.findIndex(t => t.id === action.payload.id);
        if (index !== -1) {
          state.tasks[index] = action.payload as any;
        }
        if (state.currentTask?.id === action.payload.id) {
          state.currentTask = action.payload as any;
        }
      });
    });

    // assignTask
    builder.addCase(assignTask.fulfilled, (state, action) => {
      const index = state.tasks.findIndex(t => t.id === action.payload.id);
      if (index !== -1) {
        state.tasks[index] = action.payload as any;
      }
    });
  },
});

export const { setFilters, clearCurrentTask, clearError } = taskSlice.actions;
export default taskSlice.reducer;
