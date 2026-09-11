import React from "react";
import { CheckCircle2, Clock, AlertCircle, PlayCircle, GitBranch, Cpu } from "lucide-react";

export interface TaskNode {
  id: string;
  tag: string;
  prompt: string;
  dependsOn?: string[];
  state: "queued" | "running" | "done" | "failed";
  assignedModel?: string;
  assignedProvider?: string;
  latencyMs?: number;
  lastError?: string;
}

export interface SprintPipelineProps {
  goal: string;
  status: "active" | "judging" | "done" | "failed";
  tasks: TaskNode[];
  onSelectTask?: (taskId: string) => void;
  selectedTaskId?: string;
}

export const SprintPipeline: React.FC<SprintPipelineProps> = ({
  goal,
  status,
  tasks,
  onSelectTask,
  selectedTaskId,
}) => {
  const getStatusIcon = (state: TaskNode["state"]) => {
    switch (state) {
      case "done":
        return <CheckCircle2 className="w-4 h-4 text-emerald-500" />;
      case "running":
        return <PlayCircle className="w-4 h-4 text-sky-500 animate-pulse" />;
      case "failed":
        return <AlertCircle className="w-4 h-4 text-rose-500" />;
      default:
        return <Clock className="w-4 h-4 text-slate-400" />;
    }
  };

  const getTagBadge = (tag: string) => {
    const colors: Record<string, string> = {
      code: "bg-blue-500/10 text-blue-400 border-blue-500/20",
      plan: "bg-purple-500/10 text-purple-400 border-purple-500/20",
      reasoning: "bg-amber-500/10 text-amber-400 border-amber-500/20",
      vision: "bg-emerald-500/10 text-emerald-400 border-emerald-500/20",
      chat: "bg-slate-500/10 text-slate-400 border-slate-500/20",
    };
    return (
      <span
        className={`px-2 py-0.5 text-xs font-mono rounded border ${
          colors[tag] || colors.chat
        }`}
      >
        #{tag}
      </span>
    );
  };

  return (
    <div className="flex flex-col h-full bg-background text-text p-4 space-y-4 overflow-y-auto">
      <div className="flex items-center justify-between border-b border-border pb-3">
        <div>
          <span className="text-xs font-semibold text-subtext uppercase tracking-wider">
            Active Sprint Pipeline
          </span>
          <h2 className="text-lg font-bold text-text mt-0.5">{goal}</h2>
        </div>
        <div className="flex items-center space-x-2">
          <span className="text-xs text-subtext font-mono">Status:</span>
          <span
            className={`px-2.5 py-1 text-xs font-medium rounded-full uppercase ${
              status === "done"
                ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
                : status === "failed"
                ? "bg-rose-500/10 text-rose-400 border border-rose-500/20"
                : "bg-sky-500/10 text-sky-400 border border-sky-500/20 animate-pulse"
            }`}
          >
            {status}
          </span>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        {tasks.map((task) => {
          const isSelected = selectedTaskId === task.id;
          return (
            <div
              key={task.id}
              onClick={() => onSelectTask?.(task.id)}
              className={`p-3.5 rounded-lg border transition-all cursor-pointer flex flex-col justify-between space-y-3 ${
                isSelected
                  ? "border-accent bg-accent/5 ring-1 ring-accent"
                  : "border-border bg-surface hover:border-border-hover"
              }`}
            >
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    {getStatusIcon(task.state)}
                    <span className="font-mono text-sm font-semibold text-text">
                      {task.id}
                    </span>
                  </div>
                  {getTagBadge(task.tag)}
                </div>
                <p className="text-xs text-subtext line-clamp-3 leading-relaxed">
                  {task.prompt}
                </p>
              </div>

              <div className="pt-2 border-t border-border/50 flex items-center justify-between text-xs text-subtext font-mono">
                <div className="flex items-center space-x-1.5">
                  <Cpu className="w-3.5 h-3.5 text-slate-400" />
                  <span className="truncate max-w-[140px]">
                    {task.assignedModel || "Auto (OmniRoute)"}
                  </span>
                </div>
                {task.latencyMs && (
                  <span>{(task.latencyMs / 1000).toFixed(1)}s</span>
                )}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
