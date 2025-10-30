import { App as AntApp } from 'antd'
import React, { useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import { Navigate, Route, Routes } from 'react-router-dom'
import './App.css'
import AppLayout from './components/layout/AppLayout'
import Login from './pages/Login'
import Dashboard from './pages/Dashboard'
import TaskManagement from './pages/TaskManagement'
import ProjectManagement from './pages/ProjectManagement'
import Analytics from './pages/Analytics'
import UserManagement from './pages/UserManagement'
import { RootState } from './store'
import { clearAuthState, getCurrentUser } from './store/authSlice'
import { ProtectedRoute } from './components/ProtectedRoute'
import { UserRole } from './types/user'

const App: React.FC = () => {
  const dispatch = useDispatch()
  const { isAuthenticated, user, loading } = useSelector((state: RootState) => state.auth)

  // 应用启动时验证token有效性
  useEffect(() => {
    const token = localStorage.getItem('token')
    if (token) {
      dispatch(getCurrentUser() as any).catch(() => {
        dispatch(clearAuthState() as any)
      })
    } else {
      dispatch(clearAuthState() as any)
    }
  }, [dispatch])

  // 全局加载状态
  if (loading) {
    return (
      <div style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        fontSize: '16px'
      }}>
        正在验证登录状态...
      </div>
    )
  }

  // 未认证时显示登录页面
  if (!isAuthenticated) {
    return (
      <AntApp>
        <Routes>
          {/* 重定向所有路径到登录页 */}
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<Navigate to="/login" replace />} />
        </Routes>
      </AntApp>
    )
  }

  // 已认证但用户信息为空的异常情况
  if (!user) {
    return (
      <div style={{
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        fontSize: '16px'
      }}>
        <div>用户信息加载异常</div>
        <button
          onClick={() => window.location.reload()}
          style={{ marginTop: '20px', padding: '10px 20px' }}
        >
          刷新页面
        </button>
      </div>
    )
  }

  // 已认证时的主要路由
  return (
    <AntApp>
      <Routes>
        {/* 登录页面重定向 - 已登录用户访问登录页时直接重定向到仪表板 */}
        <Route
          path="/login"
          element={<Navigate to="/dashboard" replace />}
        />

        {/* TaskFleet主应用路由 - 所有路由都需要认证 */}
        <Route path="/" element={<AppLayout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          
          {/* 仪表板 - 所有角色都可访问 */}
          <Route 
            path="dashboard" 
            element={
              <ProtectedRoute allowedRoles={[UserRole.PlatformAdmin, UserRole.ProjectManager, UserRole.TaskExecutor]}>
                <Dashboard />
              </ProtectedRoute>
            } 
          />
          
          {/* 任务管理 - 所有角色都可访问 */}
          <Route 
            path="tasks" 
            element={
              <ProtectedRoute allowedRoles={[UserRole.PlatformAdmin, UserRole.ProjectManager, UserRole.TaskExecutor]}>
                <TaskManagement />
              </ProtectedRoute>
            } 
          />
          
          {/* 项目管理 - 所有角色都可访问 */}
          <Route 
            path="projects" 
            element={
              <ProtectedRoute allowedRoles={[UserRole.PlatformAdmin, UserRole.ProjectManager, UserRole.TaskExecutor]}>
                <ProjectManagement />
              </ProtectedRoute>
            } 
          />
          
          {/* 数据分析 - 仅管理员可访问 */}
          <Route 
            path="analytics" 
            element={
              <ProtectedRoute allowedRoles={[UserRole.PlatformAdmin, UserRole.ProjectManager]}>
                <Analytics />
              </ProtectedRoute>
            } 
          />
          
          {/* 用户管理 - 仅管理员可访问 */}
          <Route 
            path="users" 
            element={
              <ProtectedRoute allowedRoles={[UserRole.PlatformAdmin, UserRole.ProjectManager]}>
                <UserManagement />
              </ProtectedRoute>
            } 
          />
        </Route>

        {/* 404页面 - 重定向到仪表板 */}
        <Route path="*" element={<Navigate to="/dashboard" replace />} />
      </Routes>
    </AntApp>
  )
}

export default App
