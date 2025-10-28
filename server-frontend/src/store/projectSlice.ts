/**
 * TaskFleet - 项目管理Store
 */

import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import projectService from '../services/projectService';
import { Project, ProjectInfo, CreateProjectRequest, UpdateProjectRequest, ProjectQueryParams } from '../types';

interface ProjectState {
  projects: ProjectInfo[];
  currentProject: ProjectInfo | null;
  loading: boolean;
  error: string | null;
  filters: ProjectQueryParams;
}

const initialState: ProjectState = {
  projects: [],
  currentProject: null,
  loading: false,
  error: null,
  filters: {},
};

// 异步Actions
export const fetchProjects = createAsyncThunk(
  'project/fetchProjects',
  async (params?: ProjectQueryParams) => {
    const response = await projectService.getProjects(params);
    return response;
  }
);

export const fetchProject = createAsyncThunk(
  'project/fetchProject',
  async (id: string) => {
    const response = await projectService.getProject(id);
    return response;
  }
);

export const createProject = createAsyncThunk(
  'project/createProject',
  async (data: CreateProjectRequest) => {
    const response = await projectService.createProject(data);
    return response;
  }
);

export const updateProject = createAsyncThunk(
  'project/updateProject',
  async ({ id, data }: { id: string; data: UpdateProjectRequest }) => {
    const response = await projectService.updateProject(id, data);
    return response;
  }
);

export const deleteProject = createAsyncThunk(
  'project/deleteProject',
  async (id: string) => {
    await projectService.deleteProject(id);
    return id;
  }
);

export const startProject = createAsyncThunk(
  'project/startProject',
  async (id: string) => {
    const response = await projectService.startProject(id);
    return response;
  }
);

export const holdProject = createAsyncThunk(
  'project/holdProject',
  async (id: string) => {
    const response = await projectService.holdProject(id);
    return response;
  }
);

export const completeProject = createAsyncThunk(
  'project/completeProject',
  async (id: string) => {
    const response = await projectService.completeProject(id);
    return response;
  }
);

export const cancelProject = createAsyncThunk(
  'project/cancelProject',
  async (id: string) => {
    const response = await projectService.cancelProject(id);
    return response;
  }
);

// Slice
const projectSlice = createSlice({
  name: 'project',
  initialState,
  reducers: {
    setFilters: (state, action: PayloadAction<ProjectQueryParams>) => {
      state.filters = action.payload;
    },
    clearCurrentProject: (state) => {
      state.currentProject = null;
    },
    clearError: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    // fetchProjects
    builder.addCase(fetchProjects.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(fetchProjects.fulfilled, (state, action) => {
      state.loading = false;
      state.projects = action.payload;
    });
    builder.addCase(fetchProjects.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to fetch projects';
    });

    // fetchProject
    builder.addCase(fetchProject.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(fetchProject.fulfilled, (state, action) => {
      state.loading = false;
      state.currentProject = action.payload;
    });
    builder.addCase(fetchProject.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to fetch project';
    });

    // createProject
    builder.addCase(createProject.pending, (state) => {
      state.loading = true;
      state.error = null;
    });
    builder.addCase(createProject.fulfilled, (state, action) => {
      state.loading = false;
      state.projects.unshift(action.payload as any);
    });
    builder.addCase(createProject.rejected, (state, action) => {
      state.loading = false;
      state.error = action.error.message || 'Failed to create project';
    });

    // updateProject
    builder.addCase(updateProject.fulfilled, (state, action) => {
      const index = state.projects.findIndex(p => p.id === action.payload.id);
      if (index !== -1) {
        state.projects[index] = action.payload as any;
      }
      if (state.currentProject?.id === action.payload.id) {
        state.currentProject = action.payload as any;
      }
    });

    // deleteProject
    builder.addCase(deleteProject.fulfilled, (state, action) => {
      state.projects = state.projects.filter(p => p.id !== action.payload);
      if (state.currentProject?.id === action.payload) {
        state.currentProject = null;
      }
    });

    // Project lifecycle actions
    [startProject, holdProject, completeProject, cancelProject].forEach(thunk => {
      builder.addCase(thunk.fulfilled, (state, action) => {
        const index = state.projects.findIndex(p => p.id === action.payload.id);
        if (index !== -1) {
          state.projects[index] = action.payload as any;
        }
        if (state.currentProject?.id === action.payload.id) {
          state.currentProject = action.payload as any;
        }
      });
    });
  },
});

export const { setFilters, clearCurrentProject, clearError } = projectSlice.actions;
export default projectSlice.reducer;
