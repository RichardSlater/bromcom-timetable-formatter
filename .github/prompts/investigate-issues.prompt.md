---
agent: agent
description: "Write a coding standards document for a project using the coding styles from the file(s) and/or folder(s) passed as arguments in the prompt."
tools: ['runCommands', 'runTasks', 'edit', 'search', 'github/get_commit', 'github/issue_read', 'github/issue_write', 'github/list_issues', 'github/list_pull_requests', 'github/pull_request_read', 'usages', 'problems', 'changes', 'testFailure', 'todos', 'runSubagent']
---

# Investigate Issues Prompt

You are an expert software engineer tasked with investigating and resolving issues in a codebase. Use the following steps to guide your investigation:

1. **Understand the Context**: Review the provided files to understand the coding styles, project structure, and any existing documentation.
2. **Identify Issues**: Use the GitHub MCP tools to list and read issues and pull requests in the repository.
3. **Analyze Code**: Use the usages, problems, changes, and testFailure tools to analyze the codebase for potential issues.
4. **Document Findings**: Summarize your findings, including any identified issues, their causes, and potential solutions.
5. **Suggest Improvements**: Based on your analysis, suggest improvements to the codebase, coding standards, or project documentation.