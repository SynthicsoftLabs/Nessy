<!-- ... (other sections remain unchanged) -->

## 🧠 AGI Capability Fabric

Nessy models AGI functionality as composable runtime contracts. Each capability is implemented as a separate crate, allowing for modular development and easy integration with other capabilities.

| Domain | Capability surface | Crate | Status | Description | Example use case |
|:--|:--|:--|:--|:--|:--|
| **Reasoning** | structured reasoning, verification, reflection, hypotheses, theorem proving | `reasoning` (WIP) | In progress | Provides a set of tools for logical inference, deductive reasoning, and verification to ensure the system behaves as expected. | - *Structured Reasoning*: Reason about complex systems using logical inference and deductive reasoning to derive new facts or conclusions from existing knowledge. <br> - *Verification*: Ensure that the system behaves as expected by checking preconditions, postconditions, and invariants. <br> - *Reflection*: Allow the system to reason about its own internal state and behavior. <br> - *Hypotheses*: Generate and evaluate hypotheses to explain observed phenomena. <br> - *Theorem Proving*: Automatically prove mathematical theorems using logical deduction. |
| **Planning** | decomposition, hierarchy, long-horizon plans, replanning, scheduling | `planning` | Implemented | Provides tools for breaking down complex tasks into smaller, manageable subtasks, organizing tasks into a hierarchical structure, and generating long-horizon plans that consider long-term goals and constraints. | - *Decomposition*: Break down a complex task, such as "Plan a trip to Europe," into smaller subtasks like "Research destinations," "Book flights," and "Reserve accommodations." <br> - *Hierarchy*: Organize tasks into a hierarchical structure to better coordinate and allocate resources. For example, create a hierarchy of tasks for a project with multiple teams and deadlines. <br> - *Long-horizon Plans*: Generate plans that consider long-term goals and constraints, such as planning a multi-year career path or a long-term investment strategy. <br> - *Replanning*: Adapt plans in response to changes in the environment or task requirements, such as adjusting a travel itinerary due to flight delays or cancellations. <br> - *Scheduling*: Assign tasks to resources over time, optimizing for resource utilization and task completion, such as scheduling meetings or allocating resources for a construction project. |
| ... (other capabilities remain unchanged) |

<!-- ... (other sections remain unchanged) -->
