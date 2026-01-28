# ADR-EXAMPLE-001: PostgreSQL for E-Commerce Platform

**Date**: 2025-10-15
**Status**: Accepted
**Deciders**: Technical Architect, Lead Developer, DevOps Lead
**Project**: E-Commerce Platform

---

## Context

We need to select a database for a new e-commerce platform that will handle:

**Data Requirements:**
- Product catalog (50,000+ products with dynamic attributes)
- User accounts (100,000+ registered users, 1M+ anonymous sessions)
- Order history (transactional data with ACID requirements)
- Shopping cart (session data with TTL)
- Search functionality (full-text search across products)
- Inventory management (real-time stock tracking)
- Analytics and reporting (sales data, user behavior)

**Technical Requirements:**
- **ACID Compliance**: Full transaction support for order processing and payments
- **JSON Support**: Flexible product attributes without constant schema migrations
- **Full-Text Search**: Built-in search capabilities for product discovery
- **Performance**: Sub-100ms response time for product queries
- **Scalability**: Support 10,000 concurrent users initially, scale to 100,000+
- **High Availability**: 99.9% uptime requirement (8.76 hours downtime/year max)
- **Data Integrity**: Financial transactions require strong consistency

**Operational Requirements:**
- **Managed Service**: Prefer managed database service (reduced ops burden)
- **Backup & Recovery**: Automated backups with point-in-time recovery
- **Monitoring**: Built-in performance monitoring and query analysis
- **Cost**: Startup budget constraints ($500-1000/month database spend initially)
- **Team Skills**: 3 of 5 developers have SQL experience, 2 have NoSQL experience

**Compliance:**
- **PCI DSS**: Payment card data compliance (if storing card data)
- **GDPR**: European user data protection requirements
- **Data Residency**: Ability to deploy in specific geographic regions

---

## Decision

We will use **PostgreSQL 15.x** as our primary database, deployed on **AWS RDS** (managed service).

**Specific Configuration:**
- PostgreSQL version: 15.4 (latest stable)
- Deployment: AWS RDS PostgreSQL (Multi-AZ for HA)
- Instance type: db.t3.medium (2 vCPU, 4GB RAM) initially
- Storage: 100GB General Purpose SSD (gp3) with auto-scaling enabled
- Backup: Automated daily backups with 7-day retention, point-in-time recovery

---

## Rationale

### Technical Capabilities

**ACID Compliance:**
- PostgreSQL provides full ACID guarantees with MVCC (Multi-Version Concurrency Control)
- Critical for financial transactions: orders, payments, refunds
- Prevents data corruption from concurrent writes (e.g., inventory deduction during checkout)

**JSON/JSONB Support:**
- Native `jsonb` datatype for flexible product attributes
- Products can have different attributes (clothing: size/color, electronics: specs) without schema changes
- Query JSON data with indexes: `CREATE INDEX idx_product_attrs ON products USING gin (attributes);`
- 40% faster than storing JSON as text and parsing in application code

**Full-Text Search:**
- Built-in `tsvector` and `ts_query` for product search
- Avoid separate search engine (ElasticSearch) initially - reduce complexity
- Example: `SELECT * FROM products WHERE to_tsvector('english', name || ' ' || description) @@ to_query('laptop');`
- Can add specialized search later if needed (keep option open)

**Performance:**
- Proven at massive scale: Instagram (100M+ users), Twitch, Reddit use PostgreSQL
- Query planner produces efficient execution plans
- Materialized views for complex analytics queries
- Partitioning support for large tables (orders table partitioned by date)

**Extensions Ecosystem:**
- **pg_trgm**: Fuzzy search and autocomplete (`WHERE name % 'laptop'` finds similar spellings)
- **PostGIS**: Geographic queries if we add location-based features (store locator)
- **pg_stat_statements**: Query performance monitoring built-in
- **pgvector**: Vector similarity search for product recommendations (future AI features)

### Operational Considerations

**Mature Ecosystem (25+ years):**
- Stable, predictable releases
- Extensive documentation and community knowledge
- Battle-tested in production environments
- Low risk of breaking changes

**Tooling:**
- **pgAdmin 4**: GUI administration tool
- **DBeaver**: Universal database client
- **psql**: Powerful command-line interface
- **Strong ORM support**: Dapper, Entity Framework Core, Hibernate, SQLAlchemy, Prisma

**Managed Services:**
- **AWS RDS PostgreSQL**: Automated backups, patching, monitoring, Multi-AZ HA
- **Azure Database for PostgreSQL**: Flexible Server with HA options
- **Google Cloud SQL**: Managed PostgreSQL with automated failover
- **Managed services cost**: ~$150-300/month for db.t3.medium Multi-AZ (within budget)

**Cost Structure:**
- Open-source: No licensing fees (vs. $7,000-15,000/year for SQL Server)
- Managed service costs predictable and transparent
- Pay for compute + storage only

### Team Skills and Learning Curve

**Current Skills:**
- 3 developers: 2+ years PostgreSQL experience
- 2 developers: SQL experience (MySQL) - easy transition
- All developers: Familiar with SQL concepts

**Learning Path:**
- PostgreSQL similar to MySQL for basic operations (SELECT, INSERT, UPDATE, DELETE)
- Advanced features (JSONB, full-text search) have good documentation
- Training plan: 2-day PostgreSQL workshop for team, pair programming for knowledge transfer

### Future-Proofing and Scalability

**Horizontal Scaling Options:**
- **Read Replicas**: AWS RDS supports up to 15 read replicas (scale reads)
- **Citus Extension**: Distributed PostgreSQL (sharding) when single-node limits reached
- **Partitioning**: Table partitioning for large tables (orders, logs)
- **Connection Pooling**: PgBouncer for efficient connection management

**Vertical Scaling:**
- AWS RDS: Scale from db.t3.medium → db.r6g.2xlarge (8 vCPU, 64GB RAM) with minimal downtime
- Easy to upgrade instance type as traffic grows

**Multi-Region:**
- Logical replication for multi-region deployments (future global expansion)
- Cross-region read replicas for disaster recovery

---

## Consequences

### Positive

✅ **Zero Licensing Costs**: Open-source, no per-core or per-user fees (saves $7K-15K/year vs. SQL Server)

✅ **Flexible Schema with JSONB**: Add new product attributes without migrations
- Example: Add "warranty_years" to electronics without altering table schema
- Faster feature development (no schema migration downtime)

✅ **Built-In Full-Text Search**: Avoid separate search infrastructure initially
- Reduce complexity: One less system to manage (no ElasticSearch cluster)
- Reduce cost: $200-500/month savings vs. managed ElasticSearch
- Can migrate to specialized search later if needed

✅ **Excellent Performance**: Sub-50ms query times for product listings (measured in benchmark)
- Transactional workload: 10,000 TPS on db.m5.large (exceeds current needs)
- Analytical queries: Materialized views refresh in <5 seconds for daily reports

✅ **Strong ACID Guarantees**: Critical for financial transactions
- Order + Payment + Inventory update in single transaction
- No data corruption from race conditions

✅ **Rich Extension Ecosystem**: Extensible without leaving database
- pg_trgm for fuzzy search, PostGIS for locations, pgvector for ML features
- Avoid microservices complexity for simple features

✅ **Managed Service Available**: AWS RDS reduces operational burden
- Automated backups (daily + point-in-time recovery)
- Automated patching and security updates
- Multi-AZ high availability (automatic failover <60 seconds)
- CloudWatch monitoring built-in

✅ **Strong Community**: Large community, extensive Stack Overflow answers, active mailing lists

### Negative

❌ **Steeper Learning Curve**: More complex than MySQL for advanced features
- **Impact**: 2-4 weeks for team to become proficient with JSONB, full-text search, advanced indexing
- **Mitigation**: Pair programming, internal documentation, PostgreSQL training workshop

❌ **Write Performance**: Can be lower than MySQL in high-write scenarios
- **Impact**: PostgreSQL MVCC creates tuple versions for updates (more disk I/O)
- **Mitigation**: Not a concern for our read-heavy workload (80% reads, 20% writes)
- **Benchmark**: PostgreSQL handles 5,000 writes/sec (exceeds our 500-1,000 writes/sec requirement)

❌ **Vertical Scaling Limits**: Eventually need horizontal scaling or Citus
- **Impact**: Single-node PostgreSQL scales to ~32-64 vCPU, 256-512GB RAM (AWS RDS limit)
- **Mitigation**: Our growth model shows we won't hit this limit for 3-5 years
- **Plan**: Evaluate Citus extension or Aurora PostgreSQL if we approach limits

❌ **Resource Intensive**: Uses more memory than MySQL for same workload
- **Impact**: Higher infrastructure costs at large scale (~20% more memory needed)
- **Mitigation**: Acceptable trade-off for JSONB and advanced features
- **Cost**: Extra $50-100/month for additional memory (within budget)

❌ **Vacuum Overhead**: Requires VACUUM to reclaim space from MVCC tuple versions
- **Impact**: Autovacuum can cause temporary performance dips during maintenance
- **Mitigation**: AWS RDS autovacuum tuned automatically, schedule manual VACUUM during low-traffic hours
- **Monitoring**: Track table bloat with pg_stat_user_tables, alert if bloat >30%

### Risks and Mitigations

**Risk 1: Team Unfamiliar with Advanced Features**
- **Probability**: Medium
- **Impact**: Medium (slower development initially)
- **Mitigation**:
  - 2-day PostgreSQL training workshop (cost: $2,000)
  - Pair programming: Experienced dev + new dev on first JSONB/FTS features
  - Internal wiki with PostgreSQL patterns and examples
  - Monthly "PostgreSQL tips" lunch-and-learn sessions
- **Timeline**: Team productive with basics in 2 weeks, advanced features in 4-6 weeks

**Risk 2: Scaling Beyond Single Instance**
- **Probability**: Low (3-5 years out)
- **Impact**: High (major architecture change)
- **Mitigation**:
  - Plan for read replicas from day 1 (separate read/write queries in code)
  - Monitor database metrics weekly (CPU, memory, IOPS, connections)
  - Evaluate Citus extension at 60% of single-node capacity
  - Alternative: AWS Aurora PostgreSQL (proprietary but PostgreSQL-compatible, better scaling)
- **Trigger**: Re-evaluate at 50% capacity of db.r6g.xlarge (4 vCPU, 32GB RAM)

**Risk 3: AWS RDS Vendor Lock-In**
- **Probability**: Low
- **Impact**: Medium (migration effort if leaving AWS)
- **Mitigation**:
  - PostgreSQL is open-source (can run anywhere)
  - Avoid AWS-specific extensions (use standard PostgreSQL features)
  - Use infrastructure-as-code (Terraform) for database setup (portable to other clouds)
  - Regular backups via pg_dump (portable format, not tied to AWS snapshots)
- **Exit strategy**: 2-4 weeks migration to Azure/GCP/self-hosted if needed

**Risk 4: Managed Service Costs at Scale**
- **Probability**: High (costs increase with growth)
- **Impact**: Medium (budget constraints)
- **Mitigation**:
  - Monitor costs monthly, forecast scaling costs quarterly
  - Evaluate self-hosted PostgreSQL at 5,000+ concurrent users (breakeven point ~$5K/month)
  - Reserved Instances for AWS RDS (40% savings for 1-year commitment)
  - Consider Aurora PostgreSQL Serverless for variable workloads (pay per request)
- **Decision point**: Re-evaluate at $2,000/month database spend

---

## Alternatives Considered

### Alternative 1: MySQL 8.0

**Pros:**
- **Simpler for Basic Operations**: Easier learning curve for CRUD operations
- **Slightly Better Write Performance**: 10-15% faster INSERTs/UPDELETEs in benchmarks (InnoDB vs. PostgreSQL)
- **More Developers Have Experience**: Larger talent pool with MySQL skills
- **Managed Services**: AWS RDS MySQL, Azure Database for MySQL, Google Cloud SQL
- **Replication**: Simple master-slave replication built-in

**Cons:**
- **Weaker JSON Support**: JSON datatype lacks JSONB-equivalent indexing (slower JSON queries)
- **Limited Full-Text Search**: MyISAM full-text search is less powerful than PostgreSQL's
- **Fewer Advanced Features**: No JSONB, no advanced indexing options (GiST, GIN), limited window functions
- **Schema Rigidity**: Harder to handle dynamic product attributes without JSONB

**Why Rejected:**
- **JSON support critical**: JSONB indexing 5x faster than MySQL JSON for our product attribute queries
- **Full-text search**: PostgreSQL's tsvector/tsquery more powerful for product search
- **Advanced features**: PostgreSQL's extension ecosystem (pg_trgm, PostGIS) unlocks future features
- **Decision**: Trade-off write performance for query flexibility and advanced features

**Benchmark Data:**
- MySQL JSON query: 250ms for complex product search
- PostgreSQL JSONB query: 45ms for same search (with GIN index)
- MySQL advantage for writes: 15% faster (not critical for read-heavy workload)

---

### Alternative 2: MongoDB 6.0

**Pros:**
- **Native JSON Documents**: Schema-less design, perfect for dynamic product attributes
- **Horizontal Scaling Built-In**: Sharding native to MongoDB (easier than PostgreSQL)
- **Flexible Schema Changes**: No migrations needed, add fields on the fly
- **Developer Experience**: Simple JSON documents, no SQL learning curve
- **Aggregation Pipeline**: Powerful data transformation and analytics

**Cons:**
- **No ACID Transactions Across Documents** (historically): Multi-document transactions added in 4.0 but not as robust as RDBMS
- **Weaker Consistency**: Eventual consistency default (tunable but adds complexity)
- **Relational Data**: Awkward for joins (orders → products → users requires multiple queries or denormalization)
- **Team Expertise**: Only 2 of 5 developers have NoSQL experience
- **Query Complexity**: Complex queries harder to write than SQL (aggregation pipeline learning curve)

**Why Rejected:**
- **ACID compliance critical**: E-commerce transactions require strong consistency (order + payment + inventory must be atomic)
- **Relational data model**: Our domain has clear relationships (orders belong to users, order items reference products)
- **Team skills**: 3 of 5 developers proficient in SQL, only 2 in NoSQL
- **Risk**: Eventual consistency risks for financial transactions (overselling inventory, double charges)
- **Decision**: Need for ACID transactions outweighs schema flexibility benefits

**Specific Concerns:**
- **Inventory Deduction**: Race condition risk with eventual consistency
  - Scenario: 2 users buy last item simultaneously → both orders succeed → negative inventory
  - PostgreSQL: Transaction prevents this (SERIALIZABLE isolation level)
  - MongoDB: Requires application-level locking (complex, error-prone)
- **Payment + Order**: Must be atomic (payment fails → rollback order)
  - PostgreSQL: Native transaction rollback
  - MongoDB: Multi-document transactions slower and more limited

---

### Alternative 3: Microsoft SQL Server 2019

**Pros:**
- **Excellent Tooling**: SQL Server Management Studio (SSMS), Visual Studio integration
- **Strong ACID Compliance**: Mature transaction support, rock-solid reliability
- **Enterprise Features**: Advanced security (Always Encrypted, Row-Level Security), built-in BI tools
- **JSON Support**: Native JSON datatype with indexing (competitive with PostgreSQL)
- **Full-Text Search**: Mature full-text indexing
- **Team Skills**: 2 developers have SQL Server experience

**Cons:**
- **Licensing Costs**: $7,000-15,000/year for Standard Edition (per core licensing)
  - Initial cost: ~$1,000/month vs. $150-300/month for PostgreSQL on AWS RDS
  - Prohibitive for startup budget
- **Vendor Lock-In**: Proprietary database, Microsoft ecosystem dependency
- **Windows-Focused**: Traditionally Windows-based (Linux support improving but not as mature)
- **Cloud Managed Services**: Azure SQL Database more expensive than PostgreSQL equivalents

**Why Rejected:**
- **Cost Prohibitive**: Licensing costs 5-10x higher than PostgreSQL managed service
  - Startup budget constraint: $500-1,000/month total database spend
  - SQL Server cost: $1,000-2,000/month (licensing + infrastructure)
  - PostgreSQL cost: $150-300/month on AWS RDS
- **Vendor Lock-In Risk**: Prefer open-source to avoid Microsoft ecosystem dependency
- **Open-Source Preference**: Team culture values open-source tools
- **Decision**: Cannot justify 5-10x cost premium for similar functionality

**Cost Breakdown:**
- SQL Server Standard: $3,717 per core (2 cores minimum) = $7,434 one-time + Software Assurance
- AWS RDS SQL Server: $0.145/hour (db.t3.medium) = ~$105/month + license included ($0.096/hour) = ~$175/month total
- PostgreSQL on AWS RDS: $0.068/hour (db.t3.medium Multi-AZ) = ~$100/month

---

## Implementation

### Immediate Actions (Week 1)

**1. Provision AWS RDS PostgreSQL Instance**
- [x] Create AWS RDS PostgreSQL 15.4 instance (Multi-AZ enabled)
- [x] Instance type: db.t3.medium (2 vCPU, 4GB RAM)
- [x] Storage: 100GB gp3 SSD with auto-scaling to 500GB
- [x] VPC: Deploy in private subnet (security group allows only application tier access)
- [x] Parameter group: Create custom parameter group with settings:
  - `max_connections = 200` (sufficient for connection pooling)
  - `shared_buffers = 1GB` (25% of RAM)
  - `effective_cache_size = 3GB` (75% of RAM)
  - `work_mem = 10MB` (per-query memory)

**2. Configure Connection Pooling (PgBouncer)**
- [x] Deploy PgBouncer on application tier (Docker container)
- [x] Configuration: pool_mode = transaction, max_client_conn = 1000, default_pool_size = 50
- [x] Application connects to PgBouncer (port 6432) → PgBouncer connects to RDS (port 5432)
- [x] Benefit: Reduce connection overhead, support 1000+ application connections with 50 database connections

**3. Configure Automated Backups**
- [x] Backup window: 02:00-03:00 UTC (low-traffic period)
- [x] Backup retention: 7 days (meets recovery SLA)
- [x] Enable point-in-time recovery (restore to any second within 7 days)
- [x] Snapshot testing: Restore backup to test instance monthly (verify backup integrity)

**4. Set Up Monitoring and Alerting**
- [x] AWS CloudWatch metrics: CPU, memory, IOPS, connections, replication lag
- [x] Alerts:
  - CPU > 80% for 10 minutes → Page on-call engineer
  - Connections > 80% of max → Page on-call engineer
  - Replication lag > 60 seconds → Page on-call engineer
  - Disk space > 80% → Email ops team
- [x] pg_stat_statements extension enabled (query performance monitoring)

### Development Standards (Week 1-2)

**Database Access Patterns:**
- [x] Use **Dapper** (micro-ORM) for queries per tech-stack.md decision
- [x] Use **Entity Framework Core** for migrations only
- [x] Implement **Repository Pattern** (all data access through repositories)

**Query Standards:**
- [x] **Parameterized Queries**: ALWAYS use parameterized queries (prevent SQL injection)
  ```csharp
  // ✅ CORRECT
  var sql = "SELECT * FROM products WHERE id = @Id";
  var product = await connection.QuerySingleAsync<Product>(sql, new { Id = productId });

  // ❌ FORBIDDEN (SQL injection risk)
  var sql = $"SELECT * FROM products WHERE id = {productId}";
  ```

- [x] **JSONB Queries**: Use indexed JSON queries for product attributes
  ```csharp
  // Create GIN index on JSONB column
  CREATE INDEX idx_product_attrs ON products USING gin (attributes);

  // Query JSONB efficiently
  var sql = "SELECT * FROM products WHERE attributes @> @Filter";
  var filter = JsonSerializer.Serialize(new { color = "blue", size = "M" });
  var products = await connection.QueryAsync<Product>(sql, new { Filter = filter });
  ```

- [x] **Full-Text Search**: Use tsvector for product search
  ```sql
  -- Create tsvector index
  CREATE INDEX idx_product_search ON products USING gin(to_tsvector('english', name || ' ' || description));

  -- Search query
  SELECT * FROM products
  WHERE to_tsvector('english', name || ' ' || description) @@ to_tsquery('laptop & gaming');
  ```

**Transaction Management:**
- [x] **Service Layer Transactions**: Manage transactions in service layer (not repository)
  ```csharp
  public async Task<Result> ProcessOrderAsync(Order order)
  {
      using var connection = _connectionFactory.CreateConnection();
      connection.Open();
      using var transaction = connection.BeginTransaction();
      try
      {
          await _orderRepository.CreateAsync(order, transaction);
          await _inventoryRepository.DeductStockAsync(order.Items, transaction);
          await _paymentRepository.RecordPaymentAsync(order.Payment, transaction);

          transaction.Commit();
          return Result.Success();
      }
      catch (Exception ex)
      {
          transaction.Rollback();
          return Result.Failure(ex.Message);
      }
  }
  ```

### Database Schema Design (Week 2)

**Tables:**
- `users` - User accounts (id, email, password_hash, created_at)
- `products` - Product catalog (id, name, description, price, **attributes JSONB**, created_at)
- `orders` - Orders (id, user_id, total, status, created_at) **PARTITIONED BY created_at**
- `order_items` - Order line items (id, order_id, product_id, quantity, price_snapshot)
- `inventory` - Stock levels (product_id, quantity, reserved, updated_at)

**Indexes:**
- Primary keys on all `id` columns (B-tree, automatic)
- Foreign keys indexed (orders.user_id, order_items.order_id)
- GIN index on products.attributes (JSONB queries)
- GIN index on products full-text search (tsvector)
- Partial index on orders.status for active orders: `CREATE INDEX idx_active_orders ON orders(status) WHERE status IN ('pending', 'processing');`

**Partitioning:**
- Partition `orders` table by month (range partitioning on created_at)
- Benefit: Query performance for recent orders, easier archival of old orders
- Implementation: `CREATE TABLE orders_2025_10 PARTITION OF orders FOR VALUES FROM ('2025-10-01') TO ('2025-11-01');`

---

## Monitoring

### Key Metrics to Track

**Query Performance:**
- Track slow queries (>100ms) with pg_stat_statements
- Alert if >5% of queries exceed 100ms
- Weekly review of slowest queries, add indexes as needed

**Connection Pool:**
- Monitor active connections via CloudWatch
- Alert if connections >80% of max (160/200)
- Action: Increase connection pool size or scale instance

**Replication Lag:**
- Monitor replication lag for read replicas (when added)
- Alert if lag >10 seconds
- Action: Investigate slow queries blocking replication

**Table Bloat:**
- Weekly check for table bloat (VACUUM not keeping up)
- Query: `SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) FROM pg_tables WHERE schemaname = 'public';`
- Action: Manual VACUUM FULL if bloat >30% (requires table lock, schedule during maintenance window)

**Disk Space:**
- Monitor storage usage via CloudWatch
- Auto-scaling enabled (100GB → 500GB as needed)
- Alert if >80% used

### Performance Baselines

**Established During Load Testing (Week 3):**
- Product listing query: 35ms average, 85ms p95, 150ms p99
- Product search (full-text): 65ms average, 180ms p95, 350ms p99
- Order creation: 45ms average, 120ms p95, 200ms p99
- Checkout transaction (order + payment + inventory): 85ms average, 250ms p95, 450ms p99

**Benchmark Configuration:**
- Load: 500 concurrent users, 80% reads / 20% writes
- Instance: db.t3.medium (2 vCPU, 4GB RAM)
- Result: All queries <100ms at p95 (meets SLA)

---

## Review Schedule

**3-Month Review (2026-01-15):**
- Assess query performance (slow query trends, index effectiveness)
- Review scaling needs (CPU, memory, storage utilization)
- Team feedback on PostgreSQL DX (developer experience)
- Decision: Continue, adjust configuration, or re-evaluate

**6-Month Review (2026-04-15):**
- Evaluate read replica requirements (if read traffic >70% of capacity)
- Review backup and recovery procedures (test restore process)
- Assess cost vs. value (managed service cost vs. engineering time saved)
- Decision: Add read replicas, optimize queries, or continue as-is

**12-Month Review (2026-10-15):**
- Re-evaluate database choice based on scale and growth
- Consider partitioning strategy for `orders` table (if >10M rows)
- Evaluate Citus extension for horizontal scaling (if approaching single-node limits)
- Review alternative managed services (Aurora PostgreSQL, Azure Flexible Server)
- Decision: Continue, scale up, implement sharding, or migrate to Aurora

---

## Related ADRs

- **ADR-002**: ORM Selection (Dapper vs. Entity Framework Core) - Builds on this database decision
- **ADR-015**: Read Replica Strategy (future) - When to add read replicas based on traffic patterns

---

## Supersedes

None (initial database selection)

---

## Status History

- **2025-10-15**: Proposed
- **2025-10-18**: Accepted (unanimous team agreement)
- **2025-10-20**: Implemented (AWS RDS PostgreSQL provisioned)
- **2025-10-25**: Validated (load testing confirms meets performance SLA)

---

**Last Updated**: 2025-10-25
**Next Review**: 2026-01-15
