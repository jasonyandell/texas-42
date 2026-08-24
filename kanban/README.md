# kanban/ — the project's action items

One file per task. **Status is the directory** — move the file, never
edit a status field:

    kanban/backlog/   not started
    kanban/doing/     in flight
    kanban/done/      finished (keep the file; it's the record)

Conventions:

- Card id = the file's kebab-case basename, stable for the card's life.
- **Link cards by greppable token, never by path** (paths change when
  status moves): write `[[card-id]]` anywhere — in cards, wiki pages,
  walt docs, commit messages. Find a card with
  `grep -r "\[\[card-id\]\]" kanban/` or by basename.
- Each card carries its own `id: [[card-id]]` line so the token always
  has at least one grep hit in the card itself.
- Cards are terse: What / Done when / Links. History and results go in
  the owning doc or wiki page, not the card.
- Tier discipline applies: a card is a work item, never evidence;
  nothing is promoted by a card moving to done/.
