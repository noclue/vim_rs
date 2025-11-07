# vim_rs MCP Documentation

This directory contains all design documentation for the vim_rs MCP server project.

## Document Index

### Project Overview

1. **[README.md](../README.md)** - Project overview and quick links
2. **[00-session-summary.md](00-session-summary.md)** - Complete summary of initial design discussion

### Architecture & Design

3. **[01-design-overview.md](01-design-overview.md)** - Complete architecture and design decisions
4. **[02-technology-stack.md](02-technology-stack.md)** - Detailed technology choices and rationale
5. **[03-embedding-workflow.md](03-embedding-workflow.md)** - How semantic search works (simplified explanation)
6. **[04-implementation-plan.md](04-implementation-plan.md)** - Phased development roadmap (9-14 weeks)

### vim_build Extension (Current Phase)

7. **[05-mcp-data-format-specification.md](05-mcp-data-format-specification.md)** ⭐
   - **Complete JSON schema definitions**
   - Output formats for managed objects, structures, enums
   - Field explanations and examples
   - **READ THIS FIRST for JSON format details**

8. **[06-vim-build-extension-plan.md](06-vim-build-extension-plan.md)** ⭐⭐
   - **Detailed implementation plan**
   - Code structure and architecture
   - Phase-by-phase tasks with code examples
   - Testing strategy
   - **READ THIS SECOND for implementation details**

9. **[07-vim-build-extension-summary.md](07-vim-build-extension-summary.md)** ⭐⭐⭐
   - **High-level summary for review**
   - Key design decisions
   - Timeline (9-10 days)
   - Risks and mitigation
   - Approval checklist
   - **READ THIS FIRST for quick overview**

## Quick Start - vim_build Extension

**Goal:** Extend vim_build to generate MCP-ready JSON data files.

**For review, read in this order:**

1. **[07-vim-build-extension-summary.md](07-vim-build-extension-summary.md)** (15 min)
   - Quick overview of what we're building
   - Key design decisions
   - Timeline and success criteria

2. **[05-mcp-data-format-specification.md](05-mcp-data-format-specification.md)** (30 min)
   - Detailed JSON schemas
   - Examples of output format
   - Understand what data we'll generate

3. **[06-vim-build-extension-plan.md](06-vim-build-extension-plan.md)** (45 min)
   - Implementation details
   - Code structure
   - Phase-by-phase tasks

**Total review time:** ~90 minutes

## Current Status

### Phase 0: Research & Prototyping ✅
- Design discussions completed
- Technology stack decided
- Architecture documented

### Phase 1: Data Generation (IN PLANNING) 📝
- JSON format specified
- vim_build extension planned
- Ready for implementation after review

### Future Phases
- Phase 2: MCP Server Core
- Phase 3: Search Implementation
- Phase 4: Documentation Integration
- Phase 5: Live vCenter Integration

## Key Decisions Summary

| Decision | Choice | Document |
|----------|--------|----------|
| Language | 100% Rust | [02-technology-stack.md](02-technology-stack.md) |
| Distribution | Single binary + data | [01-design-overview.md](01-design-overview.md) |
| Vector DB | LanceDB/Qdrant embedded | [02-technology-stack.md](02-technology-stack.md) |
| Data Format | JSON files | [05-mcp-data-format-specification.md](05-mcp-data-format-specification.md) |
| Naming | vim_rs style (not OpenAPI) | [05-mcp-data-format-specification.md](05-mcp-data-format-specification.md) |
| Description Parsing | Regex-based | [06-vim-build-extension-plan.md](06-vim-build-extension-plan.md) |

## Timeline Overview

### Completed
- Design phase: 1 day ✅

### In Progress
- vim_build extension planning: 1 day 📝

### Upcoming
- vim_build implementation: 9-10 days
- MCP server Phase 1-2: 4-6 weeks
- Documentation integration: 2-3 weeks
- Live vCenter integration: 2-3 weeks
- Polish & release: 1-2 weeks

**Total estimated:** 9-14 weeks from start to production release

## Questions & Feedback

For questions or feedback on any document:
1. Read the document thoroughly
2. Check related documents for context
3. Create specific, actionable questions
4. Reference document and section numbers

## Next Steps

1. **Review Phase**
   - Read summary document (07)
   - Review JSON format (05)
   - Review implementation plan (06)
   - Provide feedback

2. **Implementation Phase**
   - Create feature branch
   - Begin Phase 1: Foundation
   - Iterate with testing
   - Code review and merge

3. **Validation Phase**
   - Generate JSON files
   - Validate output
   - Test with sample MCP queries
   - Document any issues

## Document Status

| Document | Status | Last Updated | Needs Review |
|----------|--------|--------------|--------------|
| 00-session-summary.md | Complete | 2024-01-07 | No |
| 01-design-overview.md | Complete | 2024-01-07 | No |
| 02-technology-stack.md | Complete | 2024-01-07 | No |
| 03-embedding-workflow.md | Complete | 2024-01-07 | No |
| 04-implementation-plan.md | Complete | 2024-01-07 | No |
| 05-mcp-data-format-specification.md | Complete | 2024-01-07 | **YES** ⭐ |
| 06-vim-build-extension-plan.md | Complete | 2024-01-07 | **YES** ⭐⭐ |
| 07-vim-build-extension-summary.md | Complete | 2024-01-07 | **YES** ⭐⭐⭐ |

## Glossary

- **MCP**: Model Context Protocol - stdio-based protocol for AI tool integration
- **vim_rs**: Rust bindings for vSphere API
- **vim_build**: Code generator that creates vim_rs from OpenAPI specs
- **OpenAPI**: API specification format (JSON/YAML)
- **Managed Object**: vSphere object types (VirtualMachine, HostSystem, etc.)
- **Vector DB**: Database optimized for similarity search using embeddings
- **Embedding**: Dense vector representation of text for semantic search
- **RAG**: Retrieval Augmented Generation - combining search with LLM

## Resources

- [vim_rs repository](https://github.com/noclue/vim_rs)
- [MCP Protocol](https://modelcontextprotocol.io)
- [vSphere API Reference](https://developer.vmware.com/apis/vsphere-automation/latest/)
- [OpenAPI Specification](https://spec.openapis.org/oas/v3.0.0)
