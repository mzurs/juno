export const idlFactory = ({ IDL }) => {
	const CollectStatusesArgs = IDL.Record({ time_delta: IDL.Opt(IDL.Nat64) });
	const CronJobStatusesSatelliteConfig = IDL.Record({
		enabled: IDL.Bool,
		cycles_threshold: IDL.Opt(IDL.Nat64)
	});
	const CronJobStatuses = IDL.Record({
		mission_control_cycles_threshold: IDL.Opt(IDL.Nat64),
		satellites: IDL.Vec(IDL.Tuple(IDL.Principal, CronJobStatusesSatelliteConfig)),
		enabled: IDL.Bool,
		cycles_threshold: IDL.Opt(IDL.Nat64)
	});
	const CronJobs = IDL.Record({
		metadata: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
		statuses: CronJobStatuses
	});
	const CanisterStatusType = IDL.Variant({
		stopped: IDL.Null,
		stopping: IDL.Null,
		running: IDL.Null
	});
	const DefiniteCanisterSettings = IDL.Record({
		freezing_threshold: IDL.Nat,
		controllers: IDL.Vec(IDL.Principal),
		memory_allocation: IDL.Nat,
		compute_allocation: IDL.Nat
	});
	const CanisterStatusResponse = IDL.Record({
		status: CanisterStatusType,
		memory_size: IDL.Nat,
		cycles: IDL.Nat,
		settings: DefiniteCanisterSettings,
		idle_cycles_burned_per_day: IDL.Nat,
		module_hash: IDL.Opt(IDL.Vec(IDL.Nat8))
	});
	const SegmentStatus = IDL.Record({
		id: IDL.Principal,
		status: CanisterStatusResponse,
		metadata: IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
		status_at: IDL.Nat64
	});
	const Result = IDL.Variant({ Ok: SegmentStatus, Err: IDL.Text });
	const SegmentsStatuses = IDL.Record({
		satellites: IDL.Opt(IDL.Vec(Result)),
		mission_control: Result
	});
	const Result_1 = IDL.Variant({ Ok: SegmentsStatuses, Err: IDL.Text });
	const CollectStatuses = IDL.Record({
		cron_jobs: CronJobs,
		statuses: Result_1,
		timestamp: IDL.Nat64
	});
	const DeleteControllersArgs = IDL.Record({
		controllers: IDL.Vec(IDL.Principal)
	});
	const CronTab = IDL.Record({
		cron_jobs: CronJobs,
		updated_at: IDL.Nat64,
		mission_control_id: IDL.Principal,
		created_at: IDL.Nat64
	});
	const SetController = IDL.Record({
		metadata: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
		expires_at: IDL.Opt(IDL.Nat64)
	});
	const SetControllersArgs = IDL.Record({
		controller: SetController,
		controllers: IDL.Vec(IDL.Principal)
	});
	const SetCronTab = IDL.Record({
		cron_jobs: CronJobs,
		updated_at: IDL.Opt(IDL.Nat64),
		mission_control_id: IDL.Principal
	});
	return IDL.Service({
		collect_statuses: IDL.Func([CollectStatusesArgs], [IDL.Vec(CollectStatuses)], ['query']),
		del_controllers: IDL.Func([DeleteControllersArgs], [], []),
		del_cron_controllers: IDL.Func([DeleteControllersArgs], [], []),
		get_cron_tab: IDL.Func([], [IDL.Opt(CronTab)], ['query']),
		list_statuses: IDL.Func([], [IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Nat64, Result_1)))], ['query']),
		set_controllers: IDL.Func([SetControllersArgs], [], []),
		set_cron_controllers: IDL.Func([SetControllersArgs], [], []),
		set_cron_tab: IDL.Func([SetCronTab], [CronTab], []),
		version: IDL.Func([], [IDL.Text], ['query'])
	});
};
export const init = ({ IDL }) => {
	return [];
};