import React from "react";
import { Terminal as TerminalIcon, CheckCircle, XCircle, AlertTriangle } from "lucide-react";

export interface TestRunnerViewProps {
  taskId: string;
  command?: string;
  status: "running" | "passed" | "failed" | "idle";
  exitCode?: number;
  output: string;
  diagnostics?: string;
}

export const TestRunnerView: React.FC<TestRunnerViewProps> = ({
  taskId,
  command,
  status,
  exitCode,
  output,
  diagnostics,
}) => {
  return (
    <div className="flex flex-col h-full bg-background border-l border-border font-mono text-xs">
      <div className="flex items-center justify-between px-4 py-2.5 bg-surface border-b border-border">
        <div className="flex items-center space-x-2">
          <TerminalIcon className="w-4 h-4 text-sky-400" />
          <span className="font-semibold text-text">Test Runner — {taskId}</span>
        </div>
        <div className="flex items-center space-x-2">
          {status === "passed" && (
            <span className="flex items-center space-x-1 text-emerald-400">
              <CheckCircle className="w-3.5 h-3.5" />
              <span>PASSED (0)</span>
            </span>
          )}
          {status === "failed" && (
            <span className="flex items-center space-x-1 text-rose-400">
              <XCircle className="w-3.5 h-3.5" />
              <span>FAILED ({exitCode ?? 1})</span>
            </span>
          )}
          {status === "running" && (
            <span className="flex items-center space-x-1 text-sky-400 animate-pulse">
              <span>RUNNING...</span>
            </span>
          )}
        </div>
      </div>

      {command && (
        <div className="px-4 py-1.5 bg-background border-b border-border/50 text-subtext text-[11px]">
          <span className="text-emerald-500 mr-2">$</span>
          {command}
        </div>
      )}

      {diagnostics && status === "failed" && (
        <div className="p-3 bg-rose-500/10 border-b border-rose-500/20 text-rose-300 space-y-1">
          <div className="flex items-center space-x-1.5 font-bold">
            <AlertTriangle className="w-3.5 h-3.5 text-rose-400" />
            <span>Judge Diagnostic Summary</span>
          </div>
          <pre className="text-[11px] whitespace-pre-wrap">{diagnostics}</pre>
        </div>
      )}

      <div className="flex-1 p-4 overflow-y-auto bg-black/40 text-slate-300 font-mono leading-relaxed whitespace-pre-wrap">
        {output || "Waiting for supervisor output..."}
      </div>
    </div>
  );
};
